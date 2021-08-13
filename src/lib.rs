use std::{cell::RefCell, net::IpAddr};

use cidr::AnyIpCidr;
use neon::prelude::*;

pub struct Group {
    name: String,
    ranges: Vec<AnyIpCidr>,
}

impl Finalize for Group {}

impl Group {
    pub fn new(name: String, ips: Vec<String>) -> Self {
        let ranges: Vec<AnyIpCidr> = ips.iter().map(|x| x.parse().unwrap()).collect();
        Group { name, ranges }
    }

    pub fn contains_ip(&self, ip: &IpAddr) -> bool {
        for range in &self.ranges {
            if range.contains(&ip) {
                return true;
            }
        }
        false
    }
}

pub struct Groups {
    groups: Vec<Group>,
}

impl Finalize for Groups {}

impl Groups {
    pub fn new(groups: Vec<Group>) -> Self {
        Groups { groups }
    }

    pub fn contains_ip_group(&self, ip: String) -> Vec<String> {
        let ip = ip.parse::<IpAddr>().unwrap();
        let mut v = vec![];
        for group in &self.groups {
            if group.contains_ip(&ip) {
                v.push(group.name.clone())
            }
        }
        v
    }
}

type BoxedGroup = JsBox<Groups>;

fn groups_new(mut cx: FunctionContext) -> JsResult<BoxedGroup> {
    let mut groups = vec![];
    let input: Handle<JsArray> = cx.argument(0)?;
    let vec: Vec<Handle<JsValue>> = input.to_vec(&mut cx)?;
    for x in vec {
        let x: Handle<JsObject> = x.downcast_or_throw(&mut cx)?;
        let group_name = x
            .get(&mut cx, "name")?
            .downcast_or_throw::<JsString, FunctionContext>(&mut cx)?
            .value(&mut cx);
        let ips: Vec<String> = x
            .get(&mut cx, "ips")?
            .downcast_or_throw::<JsArray, FunctionContext>(&mut cx)?
            .to_vec(&mut cx)?
            .iter()
            .map(|ip| ip.downcast::<JsString, FunctionContext>(&mut cx).unwrap().value(&mut cx))
            .collect();
        groups.push(Group::new(group_name, ips))
    }
    Ok(cx.boxed(Groups::new(groups)))
}

fn contains_ip_groups(mut cx: FunctionContext) -> JsResult<JsArray> {
    let groups = cx.argument::<BoxedGroup>(0)?;
    let ip = cx.argument::<JsString>(1)?.value(&mut cx);
    let v= groups.contains_ip_group(ip);
    convert_vec_to_array(cx, v)
}

fn convert_vec_to_array(mut cx: FunctionContext, vec: Vec<String>) -> JsResult<JsArray> {
    let js_array = JsArray::new(&mut cx, vec.len() as u32);

    for (i, obj) in vec.iter().enumerate() {
        let js_string = cx.string(obj);
        js_array.set(&mut cx, i as u32, js_string)?;
    }
    Ok(js_array)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("groups_new", groups_new)?;
    cx.export_function("contains_ip_groups", contains_ip_groups)?;
    Ok(())
}

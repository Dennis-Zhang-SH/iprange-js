const { hello, groups_new, contains_ip_groups } = require("./index.node");

let groups = groups_new([{ name: "group1", ips: ["172.16.0.0/16"] }, { name: "group2", ips: ["192.168.1.0/24"] }, { name: "group3", ips: ["192.168.1.0/24", "172.16.0.0/16"] }]);

let check_groups_1 = contains_ip_groups(groups, "172.16.32.1");
let check_groups_2 = contains_ip_groups(groups, "192.168.1.1");
let check_groups_3 = contains_ip_groups(groups, "172.16.32.1");

console.log(check_groups_1, check_groups_2, check_groups_3);

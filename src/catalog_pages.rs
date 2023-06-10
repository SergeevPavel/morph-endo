
fn catalog_page_prefix(n: u32) -> String {
    let s = format!("{:b}", n);
    let s2: String = s.chars().rev().map(|c| {
        match c {
            '0' => 'C',
            '1' => 'F',
            _ => unreachable!()
        }
    }).collect();
    let s1: String = std::iter::repeat('C').take(s.len()).collect();
    return format!("IIPIFFCPICFPPICIIC{}IICIPPP{}IIC", s1, s2);
}

crate::entry_point!("list_catalog_page_prefixes", list_catalog_page_prefixes);
fn list_catalog_page_prefixes() {
    let ids = vec![(1337, "This catalog page"),
                   (1729, "Structure of the Funn Genome"),
                   (8, "More notes of Funn Genomics"),
                   (23, "Activating genes [encrypted]"),
                   (42, "Gene list"),
                   (112, "Some things to look out for"),
                   (10646, "Intergalactic Character Set"),
                   (85, "Field repairing your Funn"),
                   (84, "How to fix corrupted DNA [encrypted]"),
                   (2181889, "Notes on weird RNA"),
                   (5, "Synthesis of complex structures"),
                   (4405829, "Funn security features"),
                   (123456, "History note on RNA compression"),
                   (999999999, "The question to the Ultimate Answer"),
                   (999999999, "On the an...taa....v.....")];
    for (id, topic) in ids {
        println!("{} -> {:?}", topic, id);
        println!("{}", catalog_page_prefix(id));
        println!("===================================================");
    }
}
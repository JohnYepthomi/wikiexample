// #![allow(unused)]

mod Workflow {
    
    #[derive(Debug)]
    pub struct Workflow {
        name: String,
        created: String,
        classes: Vec<WfNodeClass>,
        xpaths: Vec<WfNodeXpath>,
    }

    ////////// Example Usage /////////////
    fn usage() {
        let class = WfNodeClass {
            name: "class-1".to_string(),
            node: "div".to_string(),
        };

        let xpath = WfNodeXpath {
            path: "div>span".to_string(),
            node: "div".to_string(),
            pathtype: WfNodeXpathType::Relative,
        };

        let wf_whatsapp_replies = Workflow {
            name: "WhatsappReplies".to_string(),
            created: "31-jan-2023".to_string(),
            classes: vec![class],
            xpaths: vec![xpath],
        };

        println!("{:?}", wf_whatsapp_replies.xpaths[0].pathtype);
    }
}
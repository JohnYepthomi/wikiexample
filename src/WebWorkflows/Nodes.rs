mod Nodes {
    #[derive(Debug)]
	struct WfNodeClass {
		name: String,
		node: String
	}
	
	struct WfNodeId {
		name: String,
		node: String
	}

    #[derive(Debug)]
	enum WfNodeXpathType {
		Full,
		Relative
	}

    #[derive(Debug)]
	struct WfNodeXpath {
		path: String,
		node: String,
		pathtype: WfNodeXpathType
	}

}
#[macro_export]
macro_rules! parse_insert_punctuation {
    ($content:expr) => {
        {
            let punc = Punctuation::new($content, parse_span(&current), context.next_path());
            children.insert(punc.id(), punc.into());
        }
    };
}

#[macro_export]
macro_rules! parse_insert {
    ($expr:expr, $dest:ident) => {
        {
            let node = $expr;
            $dest.push(node.id());
            children.insert(node.id(), node.into());
        }
    };
}

#[macro_export]
macro_rules! parse_set {
    ($expr:expr, $dest:ident) => {
        {
            let node = $expr;
            $dest = node.id();
            children.insert(node.id(), node.into());
        }
    };
}

#[macro_export]
macro_rules! parse_set_optional {
    ($expr:expr, $dest:ident) => {
        {
            let node = $expr;
            $dest = Some(node.id());
            children.insert(node.id(), node.into());
        }
    };
}

#[macro_export]
macro_rules! parse_container_node_variables {
    () => {
        let span = parse_span(&pair);
        let mut children: std::collections::BTreeMap<usize, crate::ast::node::Node> = std::collections::BTreeMap::new();
        let path = context.next_parent_path();
    };
}

#[macro_export]
macro_rules! parse_container_node_variables_cleanup {
    () => {
        context.pop_parent_id();
    };
}

#[macro_export]
macro_rules! parse_node_variables {
    () => {
        let span = parse_span(&pair);
        let path = context.next_path();
    };
}

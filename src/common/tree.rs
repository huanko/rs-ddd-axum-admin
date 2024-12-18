use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeNode {
    pub id: i64,
    pub name: String,
    pub children: Vec<TreeNode>,
}

pub fn build_tree(nodes: Vec<(i64, String, Option<i64>)>) -> Option<Vec<TreeNode>> {
    if nodes.is_empty() {
        return None;
    }

    let mut children_by_parent: HashMap<i64, Vec<TreeNode>> = HashMap::new();
    let mut root_nodes = Vec::new();

    // 第一步：创建所有节点，并识别根节点
    for (id, name, parent_id) in nodes {
        let node = TreeNode {
            id,
            name,
            children: Vec::new(),
        };
        
        match parent_id {
            Some(parent_id) => {
                children_by_parent.entry(parent_id)
                    .or_insert_with(Vec::new)
                    .push(node);
            }
            None => root_nodes.push(node),
        }
    }

    // 如果没有根节点，返回 None
    if root_nodes.is_empty() && !children_by_parent.is_empty() {
        return None; // 可能存在循环引用
    }

    // 递归构建树，添加循环检测
    fn build_subtree(
        node: &mut TreeNode, 
        children_by_parent: &HashMap<i64, Vec<TreeNode>>,
        visited: &mut Vec<i64>
    ) -> bool {
        if visited.contains(&node.id) {
            return false; // 检测到循环引用
        }
        
        visited.push(node.id);
        
        if let Some(children) = children_by_parent.get(&node.id) {
            node.children = children.clone();
            for child in &mut node.children {
                if !build_subtree(child, children_by_parent, visited) {
                    return false;
                }
            }
        }
        
        visited.pop();
        true
    }

    // 为每个根节点构建子树
    let mut visited = Vec::new();
    for root in &mut root_nodes {
        if !build_subtree(root, &children_by_parent, &mut visited) {
            return None; // 如果检测到循环引用，返回 None
        }
    }

    if root_nodes.is_empty() {
        None
    } else {
        Some(root_nodes)
    }
}
pub(crate) mod bootstrap;
mod common;
pub(crate) mod page;

pub(crate) use common::send_subgraph_query;

#[cfg(test)]
mod tests {
    mod it_bootstrap_query;
    mod it_page_query;
    mod testlib;
}

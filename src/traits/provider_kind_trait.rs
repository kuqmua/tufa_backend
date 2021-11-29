pub trait ProviderKindTrait {
    fn is_link_limits_enabled(&self) -> bool;
    fn is_randomize_order_mongo_link_parts_enabled(&self) -> bool;
    fn get_check_link(&self) -> &'static str;
    fn get_init_local_data_file_path(&self) -> String;
    fn get_item_handle(&self) -> Option<&'static str>;
}

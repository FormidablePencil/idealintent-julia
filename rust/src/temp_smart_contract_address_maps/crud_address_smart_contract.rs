// todo - only dealing with addresses. This will belong in a smart contract

pub fn save_in_collection_of_all(address: String, pinned: bool) {
    // private on the Secret Network to keep people from seeing all completely

}

fn delete(address: String) {

}

// no need to get, update

#[cfg(test)]
mod smart_contract_address_maps_tests {
    use crate::temp_smart_contract_address_maps::crud_address_smart_contract::save_in_collection_of_all;

    #[test]
    fn crud() {
        let ipfs_content_address = String::from("ipfs_content_address");
        // save_in_collection_of_all(ipfs_content_address,);

    }
}
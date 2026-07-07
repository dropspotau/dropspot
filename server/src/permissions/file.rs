use crate::db::{File, User};

// Whether a user can see the file
pub fn can_see_file(file: &File, user: &User) -> bool {
    // Admins and uploaders can view this file
    if file.created_by_id.is_none() {
        // Anonymous upload, anyone can view
        return true;
    }

    // Otherwise, admins can view it
    user.is_admin
}

// Whether a user can edit the file
pub fn can_update_file(file: &File, user: &User) -> bool {
    if file.created_by_id.is_none() {
        // Anonymous upload, no one can edit it
        return false;
    }

    // Otherwise, admins and the uploader can update it
    let created_by_id = file.created_by_id.unwrap();
    user.is_admin || created_by_id == user.id
}

// Whether a user can delete the file
pub fn can_delete_file(file: &File, user: &User) -> bool {
    if file.created_by_id.is_none() {
        // Anonymous upload, no one can delete it
        return false;
    }

    // Otherwise, admins and the uploader can delete it
    let created_by_id = file.created_by_id.unwrap();
    user.is_admin || created_by_id == user.id
}

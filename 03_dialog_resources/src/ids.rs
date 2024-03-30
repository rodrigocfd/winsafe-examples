//! IDs of the resources defined in "resources\example03.res".
//!
//! They are the "glue" between our Rust code and the dialog resources.

use winsafe::seq_ids;

seq_ids! {
	ICO_MAIN = 101;
}

seq_ids! {
	DLG_MAIN = 1000;
	LBL_INPUT
	TXT_INPUT
	BTN_SHOW
}

seq_ids! {
	DLG_MODAL = 2000;
	LBL_INCOMING
	TXT_INCOMING
	LBL_RETURN
	TXT_RETURN
	BTN_OK
	BTN_CANCEL
}

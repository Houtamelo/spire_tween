use godot::classes::{Button, Label, LineEdit, LinkButton, RichTextLabel, TextEdit};
use crate::extensions::property::macros::impl_do_property;
use crate::internal::*;

impl_do_property! {
	pub trait DoButtonText where Self: Button {
		#["text"]
		fn do_text(GString);
	}
}

impl_do_property! {
	pub trait DoLinkButtonText where Self: LinkButton {
		#["text"]
		fn do_text(GString);
	}
}

impl_do_property! {
	pub trait DoLabelText where Self: Label {
		#["text"]
		fn do_text(GString);
	}
}

impl_do_property! {
	pub trait DoLabelVisibleCharacters where Self: Label {
		#["visible_characters"]
		fn do_visible_characters(i64);
	}
}

impl_do_property! {
	pub trait DoLabelVisibleRatio where Self: Label {
		#["do_visible_ratio"]
		fn do_visible_ratio(f64);
	}
}

impl_do_property! {
	pub trait DoLineEditText where Self: LineEdit {
		#["text"]
		fn do_text(GString);
	}
}

impl_do_property! {
	pub trait DoLineEditPlaceholderText where Self: LineEdit {
		#["placeholder_text"]
		fn do_placeholder_text(GString);
	}
}

impl_do_property! {
	pub trait DoRichTextLabelText where Self: RichTextLabel {
		#["text"]
		fn do_text(GString);
	}
}

impl_do_property! {
	pub trait DoRichTextLabelBBCodeText where Self: RichTextLabel {
		#["bbcode_text"]
		fn do_bbcode_text(GString);
	}
}

impl_do_property! {
	pub trait DoRichTextLabelVisibleCharacters where Self: RichTextLabel {
		#["visible_characters"]
		fn do_visible_characters(i64);
	}
}

impl_do_property! {
	pub trait DoRichTextLabelPercentVisible where Self: RichTextLabel {
		#["percent_visible"]
		fn do_percent_visible(f64);
	}
}

impl_do_property! {
	pub trait DoTextEditText where Self: TextEdit {
		#["text"]
		fn do_text(GString);
	}
}
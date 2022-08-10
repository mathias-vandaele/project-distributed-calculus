//!
//!
//! This SVG manager is not necessary
//! But this is a work around for this ticket
//! https://github.com/yewstack/yew/issues/2483
//! Since it has not been shipped in the latest version
//! That is the only way I could find to render correctly the SVG
//!
//!

use indexmap::IndexMap;
use yew::virtual_dom::{Attributes, AttrValue, VNode, VTag};

pub trait Attributable {
    fn attrs(self) -> IndexMap<&'static str, AttrValue>;
}

impl Attributable for Vec<(&str, &str)> {
    fn attrs(self) -> IndexMap<&'static str, AttrValue> {
        self
            .into_iter()
            .map(|(key, value)| {
                let key = key.to_string();
                let value = value.to_string();
                let key: &'static str = Box::leak(key.into_boxed_str());
                (key, AttrValue::from(value))
            })
            .collect::<IndexMap<&str, AttrValue>>()
    }
}

///<radialGradient cx="50.0002709%" cy="50.0927553%" fx="50.0002709%" fy="50.0927553%" r="60.50407%" gradientTransform="matrix(.72146 0 0 1 .13927034 0)" id="a">
///    <stop stop-color="#AB3424" offset="0%"></stop>
///    <stop stop-color="#E14B25" offset="89.27%"></stop>
///</radialGradient>
pub fn get_radial_gradient_1() -> VTag {
    let mut radial_gradient = VTag::new("radialGradient");
    let attr: IndexMap<&str, AttrValue> = vec![("cx", "50.0002709%"), ("cy", "50.0927553%"), ("fx", "50.0002709%"), ("fy", "50.0927553%"), ("r", "60.50407%"),
                                               ("gradientTransform", "matrix(.72146 0 0 1 .13927034 0)"), ("id", "a")].attrs();

    radial_gradient.set_attributes(Attributes::IndexMap(attr));

    let mut child1 = VTag::new("stop");
    let mut child2 = VTag::new("stop");

    let attr_child_1 = vec![("stop-color", "#AB3424"), ("offset", "0%")].attrs();
    let attr_child_2 = vec![("stop-color", "#E14B25"), ("offset", "89.27%")].attrs();

    child1.set_attributes(Attributes::IndexMap(attr_child_1));
    child2.set_attributes(Attributes::IndexMap(attr_child_2));
    radial_gradient.add_child(VNode::from(child1));
    radial_gradient.add_child(VNode::from(child2));

    radial_gradient

}

///<@{"radialGradient"} cx="23.0500046%" cy="89.7531889%" fx="23.0500046%" fy="89.7531889%" r="144.103842%" gradientTransform="matrix(.46575 0 0 1 .12314393 0)" id="b">
///             <stop stop-color="#F37343" offset="32.95%"/>
///             <stop stop-color="#E14B25" offset="100%"/>
///</@>
pub fn get_radial_gradient_2() -> VTag {
    let mut radial_gradient = VTag::new("radialGradient");
    let attr: IndexMap<&str, AttrValue> = vec![("cx", "23.0500046%"), ("cy", "89.7531889%"), ("fx", "23.0500046%"), ("fy", "89.7531889%"), ("r", "144.103842%"),
                                               ("gradientTransform", "matrix(.46575 0 0 1 .12314393 0)"), ("id", "b")].attrs();

    radial_gradient.set_attributes(Attributes::IndexMap(attr));

    let mut child1 = VTag::new("stop");
    let mut child2 = VTag::new("stop");

    let attr_child_1 = vec![("stop-color", "#F37343"), ("offset", "32.95%")].attrs();
    let attr_child_2 = vec![("stop-color", "#E14B25"), ("offset", "100%")].attrs();

    child1.set_attributes(Attributes::IndexMap(attr_child_1));
    child2.set_attributes(Attributes::IndexMap(attr_child_2));
    radial_gradient.add_child(VNode::from(child1));
    radial_gradient.add_child(VNode::from(child2));

    radial_gradient

}

///<@{"radialGradient"} cx="25.1412879%" cy="-96.8960526%" fx="25.1412879%" fy="-96.8960526%" r="272.707428%" gradientTransform="matrix(.29114 0 0 1 .17821687 0)" id="c">
///    <stop stop-color="#AB3424" offset="0%"/>
///    <stop stop-color="#E14B25" offset="89.27%"/>
/// </@>
pub fn get_radial_gradient_3() -> VTag {
    let mut radial_gradient = VTag::new("radialGradient");
    let attr: IndexMap<&str, AttrValue> = vec![("cx", "25.1412879%"), ("cy", "-96.8960526%"), ("fx", "25.1412879%"), ("fy", "-96.8960526%"), ("r", "272.707428%"),
                                               ("gradientTransform", "matrix(.29114 0 0 1 .17821687 0)"), ("id", "c")].attrs();

    radial_gradient.set_attributes(Attributes::IndexMap(attr));

    let mut child1 = VTag::new("stop");
    let mut child2 = VTag::new("stop");

    let attr_child_1 = vec![("stop-color", "#AB3424"), ("offset", "0%")].attrs();
    let attr_child_2 = vec![("stop-color", "#E14B25"), ("offset", "89.27%")].attrs();

    child1.set_attributes(Attributes::IndexMap(attr_child_1));
    child2.set_attributes(Attributes::IndexMap(attr_child_2));
    radial_gradient.add_child(VNode::from(child1));
    radial_gradient.add_child(VNode::from(child2));

    radial_gradient

}

/// <@{"radialGradient"} cx="41.0110648%" cy="-29.8525399%" fx="41.0110648%" fy="-29.8525399%" r="195.629408%" gradientTransform="matrix(.27273 0 0 1 .29826266 0)" id="d">
///    <stop stop-color="#AB3424" offset="0%"/>
///    <stop stop-color="#E14B25" offset="89.27%"/>
/// </@>
pub fn get_radial_gradient_4() -> VTag {
    let mut radial_gradient = VTag::new("radialGradient");
    let attr: IndexMap<&str, AttrValue> = vec![("cx", "41.0110648%"), ("cy", "-29.8525399%"), ("fx", "41.0110648%"), ("fy", "-29.8525399%"), ("r", "195.629408%"),
                                               ("gradientTransform", "matrix(.27273 0 0 1 .29826266 0)"), ("id", "d")].attrs();

    radial_gradient.set_attributes(Attributes::IndexMap(attr));

    let mut child1 = VTag::new("stop");
    let mut child2 = VTag::new("stop");

    let attr_child_1 = vec![("stop-color", "#AB3424"), ("offset", "0%")].attrs();
    let attr_child_2 = vec![("stop-color", "#E14B25"), ("offset", "89.27%")].attrs();

    child1.set_attributes(Attributes::IndexMap(attr_child_1));
    child2.set_attributes(Attributes::IndexMap(attr_child_2));
    radial_gradient.add_child(VNode::from(child1));
    radial_gradient.add_child(VNode::from(child2));

    radial_gradient

}

///<@{"radialGradient"} cx="49.8598515%" cy="49.5197089%" fx="49.8598515%" fy="49.5197089%" r="114.868943%" gradientTransform="matrix(.32558 0 0 1 .33626464 0)" id="e">
///    <stop stop-color="#AB3424" offset="0%"/>
///    <stop stop-color="#E14B25" offset="89.27%"/>
///</@>
pub fn get_radial_gradient_5() -> VTag {
    let mut radial_gradient = VTag::new("radialGradient");
    let attr: IndexMap<&str, AttrValue> = vec![("cx", "49.8598515%"), ("cy", "49.5197089%"), ("fx", "49.8598515%"), ("fy", "49.5197089%"), ("r", "114.868943%"),
                                               ("gradientTransform", "matrix(.32558 0 0 1 .33626464 0)"), ("id", "e")].attrs();

    radial_gradient.set_attributes(Attributes::IndexMap(attr));

    let mut child1 = VTag::new("stop");
    let mut child2 = VTag::new("stop");

    let attr_child_1 = vec![("stop-color", "#AB3424"), ("offset", "0%")].attrs();
    let attr_child_2 = vec![("stop-color", "#E14B25"), ("offset", "89.27%")].attrs();

    child1.set_attributes(Attributes::IndexMap(attr_child_1));
    child2.set_attributes(Attributes::IndexMap(attr_child_2));
    radial_gradient.add_child(VNode::from(child1));
    radial_gradient.add_child(VNode::from(child2));

    radial_gradient

}

///<@{"radialGradient"} cx="25.1439394%" cy="-97.4328947%" fx="25.1439394%" fy="-97.4328947%" r="272.706126%" gradientTransform="matrix(.29114 0 0 1 .1782353 0)" id="f">
///    <stop stop-color="#AB3424" offset="0%"/>
///    <stop stop-color="#E14B25" offset="89.27%"/>
///</@>
pub fn get_radial_gradient_6() -> VTag {
    let mut radial_gradient = VTag::new("radialGradient");
    let attr: IndexMap<&str, AttrValue> = vec![("cx", "25.1439394%"), ("cy", "97.4328947%"), ("fx", "25.1439394%"), ("fy", "-97.4328947%"), ("r", "272.706126%"),
                                               ("gradientTransform", "matrix(.29114 0 0 1 .1782353 0)"), ("id", "f")].attrs();

    radial_gradient.set_attributes(Attributes::IndexMap(attr));

    let mut child1 = VTag::new("stop");
    let mut child2 = VTag::new("stop");

    let attr_child_1 = vec![("stop-color", "#AB3424"), ("offset", "0%")].attrs();
    let attr_child_2 = vec![("stop-color", "#E14B25"), ("offset", "89.27%")].attrs();

    child1.set_attributes(Attributes::IndexMap(attr_child_1));
    child2.set_attributes(Attributes::IndexMap(attr_child_2));
    radial_gradient.add_child(VNode::from(child1));
    radial_gradient.add_child(VNode::from(child2));

    radial_gradient

}

///<@{"linearGradient"} x1=".11643836%" y1="50.0814208%" x2="99.9578082%" y2="50.0814208%" id="g">
///    <stop stop-opacity="0" offset="0%"/>
///    <stop offset="100%"/>
///</@>
pub fn get_linear_gradient_1() -> VTag {
    let mut linear_gradient = VTag::new("linearGradient");
    let attr: IndexMap<&str, AttrValue> = vec![("x1", ".11643836%"), ("y1", "50.0814208%"), ("x2", "99.9578082%"), ("y2", "50.0814208%"), ("id", "g")].attrs();

    linear_gradient.set_attributes(Attributes::IndexMap(attr));

    let mut child1 = VTag::new("stop");
    let mut child2 = VTag::new("stop");

    let attr_child_1 = vec![("stop-opacity", "0"), ("offset", "0%")].attrs();
    let attr_child_2 = vec![("offset", "100%")].attrs();

    child1.set_attributes(Attributes::IndexMap(attr_child_1));
    child2.set_attributes(Attributes::IndexMap(attr_child_2));
    linear_gradient.add_child(VNode::from(child1));
    linear_gradient.add_child(VNode::from(child2));

    linear_gradient
}


///<@{"linearGradient"} x1="-.00005556%" y1="50.0000625%" x2="99.9999456%" y2="50.0000625%" id="h">
///    <stop stop-color="#F6F7FF" offset="0%"/>
///    <stop stop-color="#F2F2F9" offset="100%"/>
///</@>
pub fn get_linear_gradient_2() -> VTag {
    let mut linear_gradient = VTag::new("linearGradient");
    let attr: IndexMap<&str, AttrValue> = vec![("x1", "-.00005556%"), ("y1", "50.0000625%"), ("x2", "99.9999456%"), ("y2", "50.0000625%"), ("id", "h")].attrs();

    linear_gradient.set_attributes(Attributes::IndexMap(attr));

    let mut child1 = VTag::new("stop");
    let mut child2 = VTag::new("stop");

    let attr_child_1 = vec![("stop-color", "#F6F7FF"), ("offset", "0%")].attrs();
    let attr_child_2 = vec![("offset", "100%"), ("stop-color", "#F2F2F9")].attrs();

    child1.set_attributes(Attributes::IndexMap(attr_child_1));
    child2.set_attributes(Attributes::IndexMap(attr_child_2));
    linear_gradient.add_child(VNode::from(child1));
    linear_gradient.add_child(VNode::from(child2));

    linear_gradient
}


///<@{"linearGradient"} x1="50%" y1="89.9963563%" x2="50%" y2="18.1270519%" id="i">
///    <stop stop-color="#EE503B" offset="0%"/>
///    <stop stop-color="#EE503B" stop-opacity="0" offset="100%"/>
///</@>

pub fn get_linear_gradient_3() -> VTag {
    let mut linear_gradient = VTag::new("linearGradient");
    let attr: IndexMap<&str, AttrValue> = vec![("x1", "50%"), ("y1", "89.9963563%"), ("x2", "50%"), ("y2", "18.1270519%"), ("id", "i")].attrs();

    linear_gradient.set_attributes(Attributes::IndexMap(attr));

    let mut child1 = VTag::new("stop");
    let mut child2 = VTag::new("stop");

    let attr_child_1 = vec![("stop-color", "#EE503B"), ("offset", "0%")].attrs();
    let attr_child_2 = vec![("offset", "100%"), ("stop-color", "#EE503B"), ("stop-opacity","0")].attrs();

    child1.set_attributes(Attributes::IndexMap(attr_child_1));
    child2.set_attributes(Attributes::IndexMap(attr_child_2));
    linear_gradient.add_child(VNode::from(child1));
    linear_gradient.add_child(VNode::from(child2));

    linear_gradient
}
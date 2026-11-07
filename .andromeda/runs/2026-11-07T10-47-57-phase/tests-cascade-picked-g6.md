@account n=1 entity=quality-gates-coverage-targets#2.1 v=carried
why: the scope ports part of upstream `286874cc1` and names its `keyboard_compat.rs` removal as not ported, the fork this entity ties the coverage exclusion to, and the new buffer module and the rewritten render sites move dioxus-native-dom's reading against its 83 % floor
@account n=2 entity=quality-gates-coverage-targets#3.1 v=carried
why: the chunk re-asserts unchanged behaviour through the `stand_*` checks, which the zero-failure `stand-repeat` leg runs three times with no retry over the new buffered render path

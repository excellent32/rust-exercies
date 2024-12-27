use web_sys::HtmlInputElement;
use yew::prelude::*;

pub struct Login{
    account_ref: NodeRef,
    pwd_ref: NodeRef,
    login_state: LoginState,
    show_error: bool,
}

pub enum LoginMsg {
    Login,
    ThirdLogin(ThirdLoginType),
    Success(AttrValue),
    Failed,
    OnEnterKeyDown(SubmitEvent),
    SwitchLanguage(Event),
    SwitchTheme(Rc<ThemeState>),
}

pub enum LoginState{
    Logining,
    Nothing,
}

impl Components for Login {
    
}
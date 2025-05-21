#![allow(dead_code)]
use serde::Serialize;

pub trait HasRefResult<'a, RT>
where
    RT: Serialize,
{
    fn result_ref(&'a self) -> &'a RT;
}

pub trait HasOwnedResult<RT>
where
    RT: Serialize,
{
    fn result(self) -> RT;
}

#[derive(Serialize)]
#[serde(bound = "RT: Serialize")]
#[serde(untagged)]
pub enum TaskResultEnum<'a, RT>
where
    RT: Serialize,
{
    Ref(TaskResultRef<'a, RT>),
    Owned(TaskResult<RT>),
}

#[derive(Serialize)]
pub struct TaskResultRef<'a, RT>
where
    RT: Serialize,
{
    pub msg: Option<&'a str>,
    pub result: &'a RT,
}

#[derive(Serialize)]
pub struct TaskResult<RT>
where
    RT: Serialize,
{
    pub msg: Option<String>,
    pub result: RT,
}

pub struct TaskInnerResult<OT> {
    pub msg: Option<String>,
    pub(crate) op: OT,
}

impl<OT> TaskInnerResult<OT> {
    pub fn to_ref_result<'a, RT>(&'a self) -> TaskResultEnum<'a, RT>
    where
        OT: HasRefResult<'a, RT>,
        RT: Serialize,
    {
        TaskResultEnum::Ref(TaskResultRef {
            msg: self.msg.as_deref(),
            result: self.op.result_ref(),
        })
    }

    pub fn to_result<RT>(self) -> TaskResultEnum<'static, RT>
    where
        OT: HasOwnedResult<RT>,
        RT: Serialize,
    {
        TaskResultEnum::Owned(TaskResult {
            msg: self.msg.clone(),
            result: self.op.result(),
        })
    }
}

#[macro_use]
extern crate napi_derive;

use serde_json;
use serde::{Deserialize, Serialize};
use rustyscript::{json_args, Module, Runtime};
use std::collections::HashMap;
use uuid::Uuid;

type Handlers = HashMap<String, Module>;

static mut HANDLERS: Option<Handlers> = None;

#[napi(object)]
#[derive(Serialize, Deserialize)]
pub struct ExecResult {
  pub code: u16,
  pub result: String,
  pub message: String,
}

/// create a handler
/// # Examples
/// ```
/// let handler = create_handler("console.log('hello world')");
/// ```
#[napi]
pub unsafe fn create_handler(code: String) -> ExecResult {
  let uuid: String = Uuid::new_v4().to_string();
  let module = Module::new(
    format!("{}.js", uuid),
    &format!(
      "export default async (params) => {{
        params = JSON.parse(params);
        let res = await {code}(...params)
        try {{
          return JSON.stringify({{
            result: JSON.stringify(res),
            code: 0,
            message: 'Execution Succeeded'
          }});
        }} catch (error) {{
          res = JSON.stringify({{
            result: '',
            code: 10002,
            message: 'The return value is invalid. The return value should be able to be formatted as JSON'
          }});
        }}
      }}",
      code = code
    ),
  );
  match HANDLERS.as_mut() {
    Some(handler_map) => {
      handler_map.insert(uuid.clone(), module);
    }
    None => {
      let mut handler_map = HashMap::new();
      handler_map.insert(uuid.clone(), module);
      HANDLERS = Some(handler_map);
    }
  }
  ExecResult {
    code: 0,
    result: uuid,
    message: "Execution Succeeded".to_string(),
  }
}

/// execute a handler
/// # Examples
/// ```
/// let result = exec_handler("handler_uuid", "arg1, arg2");
/// ```
/// # Arguments
/// * `uuid` - the uuid of the handler
/// * `args` - the arguments to pass to the handler, Receive a string type parameter.
/// return ExecResult
/// * `code` - the code of the result
/// * `result` -  Execute JS code and wait for the return result. The return value of this JavaScript code must be able to be formatted with JSON
/// * `message` - the message of the result
#[napi]
fn exec_handler(uuid: String, args: String) -> ExecResult {
  let error_result = ExecResult {
    code: 10001,
    result: "".to_string(),
    message: "No handler found".to_string(),
  };
  unsafe {
    match HANDLERS.as_ref() {
      Some(handler_map) => match handler_map.get(&uuid) {
        Some(module) => {
          let res: String =
            Runtime::execute_module(&module, vec![], Default::default(), json_args!(args)).unwrap();
          let result: ExecResult = serde_json::from_str(&res).unwrap();
          result
        }
        None => {
          return error_result;
        }
      },
      None => error_result,
    }
  }
}

/// remove a handler
/// # Examples
/// ```
/// let result = remove_handler("handler_uuid");
/// ```
#[napi]
pub unsafe fn remove_handler(uuid: String) -> ExecResult {
  let error_result = ExecResult {
    code: 10001,
    result: "".to_string(),
    message: "No handler found".to_string(),
  };
  match HANDLERS.as_mut() {
    Some(handler_map) => match handler_map.remove(&uuid) {
      Some(_) => ExecResult {
        code: 0,
        result: uuid,
        message: "Execution Succeeded".to_string(),
      },
      None => error_result,
    },
    None => error_result,
  }
}

/// Execute JS code and wait for the return result.
/// # Examples
/// ```
/// let result = sync_exec("async function test() { return 'hello world'; }");
/// ```
/// The return value of this JavaScript code must be able to be formatted with JSON
#[napi]
pub fn sync_exec(code: String, args: String) -> ExecResult {
  let module = Module::new(
    "temp.js",
    &format!(
      "export default async (params) => {{
        params = JSON.parse(params);
        let res = await {code}(...params)
        try {{
          return JSON.stringify({{
            result: JSON.stringify(res),
            code: 0,
            message: 'Execution Succeeded'
          }});
        }} catch (error) {{
          res = JSON.stringify({{
            result: '',
            code: 10002,
            message: 'The return value is invalid. The return value should be able to be formatted as JSON'
          }});
        }}
      }}",
      code = code
    ),
  );

  let res: String = Runtime::execute_module(&module, vec![], Default::default(), json_args!(args)).unwrap();
  let result: ExecResult = serde_json::from_str(&res).unwrap();
  result
}

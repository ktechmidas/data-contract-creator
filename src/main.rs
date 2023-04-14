use serde::{Serialize, Deserialize};
use yew::{html, Component, Html, Event, InputEvent, FocusEvent, TargetCast};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct DocumentType {
    name: String,
    properties: Vec<Property>,
    indices: Vec<Index>,
    required: Vec<String>,
    additionalProperties: bool,
    comment: String
}

impl Default for DocumentType {
    fn default() -> Self {
        Self {
            name: String::new(),
            properties: vec![],
            indices: vec![],
            required: vec![],
            additionalProperties: false,
            comment: String::new()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct Property {
    name: String,
    data_type: DataType,
    required: bool
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct Index {
    name: String,
    data_type: DataType,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
enum DataType {
    #[default]
    String,
    Integer,
    Array,
    Object,
    Number,
    Boolean
}

struct Model {
    document_types: Vec<DocumentType>,
    json_object: Vec<String>,
}

enum Msg {
    AddDocumentType,
    AddProperty(usize),
    AddIndex(usize),
    RemoveDocumentType(usize),
    RemoveDocumentTypeComment(usize),
    RemoveProperty(usize, usize),
    RemoveIndex(usize, usize),
    Submit,
    UpdateName(usize, String),
    UpdateComment(usize, String),
    UpdatePropertyName(usize, usize, String),
    UpdateIndexName(usize, usize, String),
    UpdatePropertyType(usize, usize, String),
    UpdateIndexType(usize, usize, String),
    UpdatePropertyRequired(usize, usize, bool)
}

impl Model {
    fn add_document_type(&mut self) {
        self.document_types.push(Default::default());
    }

    fn remove_document_type(&mut self, index: usize) {
        self.document_types.remove(index);
    }

    fn add_property(&mut self, index: usize) {
        self.document_types[index].properties.push(Default::default());
    }

    fn add_index(&mut self, index: usize) {
        self.document_types[index].indices.push(Default::default());
    }

    fn remove_doc_type_comment(&mut self, index: usize) {
        self.document_types[index].comment = String::from("");
    }

    fn remove_property(&mut self, doc_index: usize, prop_index: usize) {
        self.document_types[doc_index].properties.remove(prop_index);
    }

    fn remove_index(&mut self, doc_index: usize, index_index: usize) {
        self.document_types[doc_index].indices.remove(index_index);
    }

    fn generate_json_object(&mut self) -> Vec<String> {
        let mut json_arr = Vec::new();
        for doc_type in &mut self.document_types {
            let mut props_arr = Vec::new();
            for prop in &doc_type.properties {
                let prop_obj = json!({
                    "name": prop.name,
                    "type": match prop.data_type {
                        DataType::String => "string",
                        DataType::Integer => "integer",
                        DataType::Array => "array",
                        DataType::Object => "object",
                        DataType::Number => "number",
                        DataType::Boolean => "bool",
                    }
                });
                props_arr.push(prop_obj);
                if prop.required {
                    if !doc_type.required.contains(&prop.name) {
                        doc_type.required.push(prop.name.clone());
                    }
                } else {
                    if doc_type.required.contains(&prop.name) {
                        doc_type.required.retain(|x| x != &prop.name);
                    }
                }
            }
            let mut indices_arr = Vec::new();
            for index in &doc_type.indices {
                let index_obj = json!({
                    "name": index.name,
                    "unique": match index.data_type {
                        DataType::String => "string",
                        DataType::Integer => "integer",
                        DataType::Array => "array",
                        DataType::Object => "object",
                        DataType::Number => "number",
                        DataType::Boolean => "bool",
                    }
                });
                indices_arr.push(index_obj);
            }
            let doc_obj = json!({
                doc_type.name.clone(): {
                "properties": props_arr,
                "indices": indices_arr,
                "required": doc_type.required,
                "additionalProperties": false,
                "comment": doc_type.comment
            }});
            let formatted_doc_obj = &doc_obj.to_string()[1..doc_obj.to_string().len()-1];
            json_arr.push(formatted_doc_obj.to_string());
        }
        json_arr
    }    

    fn update_name(&mut self, index: usize, name: String) {
        self.document_types[index].name = name;
    }

    fn update_comment(&mut self, index: usize, comment: String) {
        self.document_types[index].comment = comment;
    }

    fn update_property_name(&mut self, doc_index: usize, prop_index: usize, name: String) {
        self.document_types[doc_index].properties[prop_index].name = name;
    }

    fn update_index_name(&mut self, doc_index: usize, index_index: usize, name: String) {
        self.document_types[doc_index].indices[index_index].name = name;
    }

    fn update_property_type(&mut self, doc_index: usize, prop_index: usize, data_type: String) {
        let data_type = match data_type.as_str() {
            "string" => DataType::String,
            "integer" => DataType::Integer,
            "array" => DataType::Array,
            "object" => DataType::Object,
            "number" => DataType::Number,
            "bool" => DataType::Boolean,
            _ => unreachable!(),
        };
        self.document_types[doc_index].properties[prop_index].data_type = data_type;
    }

    fn update_index_type(&mut self, doc_index: usize, index_index: usize, data_type: String) {
        let data_type = match data_type.as_str() {
            "string" => DataType::String,
            "integer" => DataType::Integer,
            "array" => DataType::Array,
            "object" => DataType::Object,
            "number" => DataType::Number,
            "bool" => DataType::Boolean,
            _ => unreachable!(),
        };
        self.document_types[doc_index].indices[index_index].data_type = data_type;
    }

    fn update_property_required(&mut self, doc_index: usize, prop_index: usize, required: bool) {
        self.document_types[doc_index].properties[prop_index].required = required;
    }

    fn view_document_types(&self, ctx: &yew::Context<Self>) -> Html {
        html! {
            <div>
                {for (0..self.document_types.len()).map(|i| self.view_document_type(i, ctx))}
            </div>
        }
    }

    fn view_document_type(&self, index: usize, ctx: &yew::Context<Self>) -> Html {
        html! {
            <div>
                <div>
                    <h2>{format!("Doc type {}", index+1)}</h2>
                    <h3>{"Name"}</h3>
                    <input type="text" placeholder="Name" value={self.document_types[index].name.clone()} onblur={ctx.link().callback(move |e: FocusEvent| Msg::UpdateName(index, e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().value()))} />
                    <button onclick={ctx.link().callback(move |_| Msg::RemoveDocumentType(index))}>{"Remove"}</button>
                </div>
                <div>
                    <h3>{"Comment"}</h3>
                    <input type="text" placeholder="Comment" value={self.document_types[index].comment.clone()} onblur={ctx.link().callback(move |e: FocusEvent| Msg::UpdateComment(index, e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().value()))} />
                    <button onclick={ctx.link().callback(move |_| Msg::RemoveDocumentTypeComment(index))}>{"Remove"}</button>
                </div>
                <div>
                    <h3>{"Properties"}</h3>
                    <table>
                        <thead>
                            <tr>
                                <th>{if self.document_types[index].properties.len() > 0 {"Name"} else {""}}</th>
                                <th>{if self.document_types[index].properties.len() > 0 {"Type"} else {""}}</th>
                                <th>{if self.document_types[index].properties.len() > 0 {"Required"} else {""}}</th>
                                <th>{if self.document_types[index].properties.len() > 0 {"Remove"} else {""}}</th>
                            </tr>
                        </thead>
                        <tbody>
                            {for (0..self.document_types[index].properties.len()).map(|i| self.view_property(index, i, ctx))}
                            <tr>
                                <td><button onclick={ctx.link().callback(move |_| Msg::AddProperty(index))}>{"+"}</button></td>
                            </tr>
                        </tbody>
                    </table>
                </div>
                <div>
                    <h3>{"Indices"}</h3>
                    <table>
                        <thead>
                            <tr>
                                <th>{if self.document_types[index].indices.len() > 0 {"Name"} else {""}}</th>
                                <th>{if self.document_types[index].indices.len() > 0 {"Unique"} else {""}}</th>
                                <th>{if self.document_types[index].indices.len() > 0 {"Remove"} else {""}}</th>
                            </tr>
                        </thead>
                        <tbody>
                            {for (0..self.document_types[index].indices.len()).map(|i| self.view_index(index, i, ctx))}
                            <tr>
                                <td><button onclick={ctx.link().callback(move |_| Msg::AddIndex(index))}>{"+"}</button></td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        }
    }

    fn view_property(&self, doc_index: usize, prop_index: usize, ctx: &yew::Context<Self>) -> Html {
        let data_type_options = vec!["string", "integer", "array", "object", "number", "boolean"];
        let selected_data_type = match self.document_types[doc_index].properties[prop_index].data_type {
            DataType::String => String::from("string"),
            DataType::Integer => String::from("integer"),
            DataType::Array => String::from("array"),
            DataType::Object => String::from("object"),
            DataType::Number => String::from("number"),
            DataType::Boolean => String::from("bool"),
        };
        html! {
            <tr>
                <td><input type="text" placeholder="Property name" value={self.document_types[doc_index].properties[prop_index].name.clone()} oninput={ctx.link().callback(move |e: InputEvent| Msg::UpdatePropertyName(doc_index, prop_index, e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().value()))} /></td>
                <td>
                    <select onchange={ctx.link().callback(move |e: Event| Msg::UpdatePropertyType(doc_index, prop_index, match e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().value().as_str() {
                        "string" => String::from("string"),
                        "integer" => String::from("integer"),
                        "array" => String::from("array"),
                        "object" => String::from("object"),
                        "number" => String::from("number"),
                        "boolean" => String::from("bool"),
                        _ => panic!("Invalid data type selected"),
                    }))}>
                        {for data_type_options.iter().map(|option| html! {
                            <option value={*option} selected={option==&selected_data_type}>{*option}</option>
                        })}
                    </select>
                </td>
                <td><input type="checkbox" checked={self.document_types[doc_index].properties[prop_index].required} onchange={ctx.link().callback(move |e: Event| Msg::UpdatePropertyRequired(doc_index, prop_index, e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().checked()))} /></td>
                <td><button onclick={ctx.link().callback(move |_| Msg::RemoveProperty(doc_index, prop_index))}>{"Remove"}</button></td>
            </tr>
        }
    }

    fn view_index(&self, doc_index: usize, index_index: usize, ctx: &yew::Context<Self>) -> Html {
        let data_type_options = vec!["string", "integer", "array", "object", "number", "boolean"];
        let selected_data_type = match self.document_types[doc_index].indices[index_index].data_type {
            DataType::String => String::from("string"),
            DataType::Integer => String::from("integer"),
            DataType::Array => String::from("array"),
            DataType::Object => String::from("object"),
            DataType::Number => String::from("number"),
            DataType::Boolean => String::from("boolean"),
        };
        html! {
            <tr>
                <td><input type="text" placeholder="Index name" value={self.document_types[doc_index].indices[index_index].name.clone()} oninput={ctx.link().callback(move |e: InputEvent| Msg::UpdateIndexName(doc_index, index_index, e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().value()))} /></td>
                <td>
                    <select onchange={ctx.link().callback(move |e: Event| Msg::UpdateIndexType(doc_index, index_index, match e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().value().as_str() {
                        "string" => String::from("string"),
                        "integer" => String::from("integer"),
                        "array" => String::from("array"),
                        "object" => String::from("object"),
                        "number" => String::from("number"),
                        "boolean" => String::from("bool"),
                        _ => panic!("Invalid data type selected"),
                    }))}>
                        {for data_type_options.iter().map(|option| html! {
                            <option value={*option} selected={option==&selected_data_type}>{option}</option>
                        })}
                    </select>
                </td>
                <td><button onclick={ctx.link().callback(move |_| Msg::RemoveIndex(doc_index, index_index))}>{"Remove"}</button></td>
            </tr>
        }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            document_types: vec![],
            json_object: vec![],
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddDocumentType => {
                self.add_document_type();
                true
            }
            Msg::AddProperty(index) => {
                self.add_property(index);
                true
            }
            Msg::AddIndex(index) => {
                self.add_index(index);
                true
            }
            Msg::RemoveDocumentType(index) => {
                self.remove_document_type(index);
                true
            }
            Msg::RemoveDocumentTypeComment(index) => {
                self.remove_doc_type_comment(index);
                true
            }
            Msg::RemoveProperty(doc_index, prop_index) => {
                self.remove_property(doc_index, prop_index);
                true
            }
            Msg::RemoveIndex(doc_index, index_index) => {
                self.remove_index(doc_index, index_index);
                true
            }
            Msg::Submit => {
                self.json_object = Some(self.generate_json_object()).unwrap();
                true
            }
            Msg::UpdateName(index, name) => {
                self.update_name(index, name);
                true
            }
            Msg::UpdateComment(index, comment) => {
                self.update_comment(index, comment);
                true
            }
            Msg::UpdatePropertyName(doc_index, prop_index, name) => {
                self.update_property_name(doc_index, prop_index, name);
                true
            }
            Msg::UpdateIndexName(doc_index, index_index, name) => {
                self.update_index_name(doc_index, index_index, name);
                true
            }
            Msg::UpdatePropertyType(doc_index, prop_index, data_type) => {
                self.update_property_type(doc_index, prop_index, data_type);
                true
            }
            Msg::UpdateIndexType(doc_index, index_index, data_type) => {
                self.update_index_type(doc_index, index_index, data_type);
                true
            }
            Msg::UpdatePropertyRequired(doc_index, prop_index, required) => {
                self.update_property_required(doc_index, prop_index, required);
                true
            }
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {        
        // html
        html! {
            <div>
                <h1>{"Data Contract Creator"}</h1>
                
                // show input fields
                {self.view_document_types(ctx)}

                // add input fields for another document type and add one to Self::document_types
                <p><button onclick={ctx.link().callback(|_| Msg::AddDocumentType)}>{"Add document type"}</button></p>

                // look at document_types and generate json object from it
                <p><button onclick={ctx.link().callback(|_| Msg::Submit)}>{"Submit"}</button></p>

                // format and display json object
                <p>
                {if self.json_object.len() != 0 as usize {
                    let s = &self.json_object.join(",");
                    let new_s = format!("{{{}}}", s);
                    new_s.to_string()
                } else { "".to_string() }}
                </p>

            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Model>::new().render();
}

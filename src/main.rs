use serde::{Serialize, Deserialize};
use yew::{html, Component, Html, Event, InputEvent, FocusEvent, TargetCast};
use serde_json::{json, Map, Value};
use web_sys::{HtmlSelectElement, console};

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
    required: bool,
    description: Option<String>,
    comment: Option<String>,
    min_length: Option<u32>,  // For String data type
    max_length: Option<u32>,  // For String data type
    pattern: Option<String>,  // For String data type
    format: Option<String>,   // For String data type
    minimum: Option<i32>,     // For Integer data type
    maximum: Option<i32>,     // For Integer data type
    byte_array: Option<bool>,  // For Array data type
    min_items: Option<u32>,    // For Array data type
    max_items: Option<u32>,    // For Array data type
    properties: Option<Box<Vec<Property>>>, // For Object data type
    min_properties: Option<u32>, // For Object data type
    max_properties: Option<u32>, // For Object data type
    rec_required: Option<Vec<String>>, // For Object data type
    additional_properties: Option<bool>, // For Object data type
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct Index {
    name: String,
    properties: Vec<IndexProperties>,
    unique: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct IndexProperties(String, String);

impl Default for IndexProperties {
    fn default() -> Self {
        Self {
            0: String::from(""),
            1: String::from("asc")
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
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
    // General
    Submit,
    AddDocumentType,
    RemoveDocumentType(usize),
    AddProperty(usize),
    RemoveProperty(usize, usize),
    AddIndex(usize),
    RemoveIndex(usize, usize),
    AddIndexProperty(usize, usize),
    UpdateName(usize, String),
    UpdateComment(usize, String),
    UpdatePropertyName(usize, usize, String),
    UpdateIndexName(usize, usize, String),
    UpdatePropertyType(usize, usize, String),
    UpdateIndexUnique(usize, usize, bool),
    UpdateIndexSorting(usize, usize, usize, String),
    UpdatePropertyRequired(usize, usize, bool),
    UpdatePropertyDescription(usize, usize, String),
    UpdatePropertyComment(usize, usize, String),
    UpdateIndexProperty(usize, usize, usize, String),

    // Optional property parameters
    UpdateStringPropertyMinLength(usize, usize, u32),
    UpdateStringPropertyMaxLength(usize, usize, u32),
    UpdateStringPropertyPattern(usize, usize, String),
    UpdateStringPropertyFormat(usize, usize, String),
    UpdateIntegerPropertyMinimum(usize, usize, i32),
    UpdateIntegerPropertyMaximum(usize, usize, i32),
    UpdateArrayPropertyByteArray(usize, usize, bool),
    UpdateArrayPropertyMinItems(usize, usize, u32),
    UpdateArrayPropertyMaxItems(usize, usize, u32),
    UpdateObjectPropertyMinProperties(usize, usize, u32),
    UpdateObjectPropertyMaxProperties(usize, usize, u32),

    // Recursive properties
    AddRecProperty(usize, usize),
    RemoveRecProperty(usize, usize, usize),
    UpdateRecPropertyType(usize, usize, usize, String),
    UpdateRecPropertyName(usize, usize, usize, String),
    UpdateRecPropertyRequired(usize, usize, usize, bool),
    UpdateRecPropertyDescription(usize, usize, usize, String),
    UpdateRecPropertyComment(usize, usize, usize, String),
    UpdateStringRecPropertyMinLength(usize, usize, usize, u32),
    UpdateStringRecPropertyMaxLength(usize, usize, usize, u32),
    UpdateStringRecPropertyPattern(usize, usize, usize, String),
    UpdateStringRecPropertyFormat(usize, usize, usize, String),
    UpdateIntegerRecPropertyMaximum(usize, usize, usize, i32),
    UpdateIntegerRecPropertyMinimum(usize, usize, usize, i32),
    UpdateArrayRecPropertyByteArray(usize, usize, usize, bool),
    UpdateArrayRecPropertyMinItems(usize, usize, usize, u32),
    UpdateArrayRecPropertyMaxItems(usize, usize, usize, u32),
    UpdateObjectRecPropertyMaxProperties(usize, usize, usize, u32),
    UpdateObjectRecPropertyMinProperties(usize, usize, usize, u32),
}

impl Model {

    fn view_document_types(&self, ctx: &yew::Context<Self>) -> Html {
        html! {
            <div>
                {for (0..self.document_types.len()).map(|i| self.view_document_type(i, ctx))}
            </div>
        }
    }

    fn view_document_type(&self, index: usize, ctx: &yew::Context<Self>) -> Html {
        html! {
            <>
            <div class="input-container">
                <div>
                    <h2>{format!("Document type {}", index+1)}</h2>
                    <h3>{"Name"}</h3>
                    <input type="text" placeholder="Name" value={self.document_types[index].name.clone()} onblur={ctx.link().callback(move |e: FocusEvent| Msg::UpdateName(index, e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().value()))} />
                </div>
                <div>
                    <h3>{"Properties"}</h3>
                    <table>
                        <tbody>
                            {for (0..self.document_types[index].properties.len()).map(|i| self.view_property(index, i, ctx))}
                            <tr>
                                <td><button class="button" onclick={ctx.link().callback(move |_| Msg::AddProperty(index))}>{"Add property"}</button></td>
                            </tr>
                        </tbody>
                    </table>
                </div>
                <div>
                    <h3>{"Indices"}</h3>
                    <table>
                        <tbody>
                            {for (0..self.document_types[index].indices.len()).map(|i| self.view_index(index, i, ctx))}
                            <tr>
                                <td><button class="button" onclick={ctx.link().callback(move |_| Msg::AddIndex(index))}>{"Add index"}</button></td>
                            </tr>
                        </tbody>
                    </table>
                </div>
                <div>
                    <h3>{"Comment"}</h3>
                    <input type="text2" placeholder="Comment" value={self.document_types[index].comment.clone()} onblur={ctx.link().callback(move |e: FocusEvent| Msg::UpdateComment(index, e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().value()))} />
                </div>
                <br/>
                <div>
                <button class="button" onclick={ctx.link().callback(move |_| Msg::RemoveDocumentType(index))}>{"Remove document type"}</button>
                </div>
            </div>
            <br/>
            </>
        }
    }

    fn view_property(&self, doc_index: usize, prop_index: usize, ctx: &yew::Context<Self>) -> Html {
        let data_type_options = vec!["String", "Integer", "Array", "Object", "Number", "Boolean"];
        let selected_data_type = match self.document_types[doc_index].properties[prop_index].data_type {
            DataType::String => String::from("String"),
            DataType::Integer => String::from("Integer"),
            DataType::Array => String::from("Array"),
            DataType::Object => String::from("Object"),
            DataType::Number => String::from("Number"),
            DataType::Boolean => String::from("Boolean"),
        };
        let additional_properties = self.render_additional_properties(&selected_data_type, doc_index, prop_index, ctx);
        html! {
            <>
                <tr>
                    <th>{"Name"}</th>
                    <th>{"Type"}</th>
                    <th>{"Required"}</th>
                </tr>
                <tr>
                    <td><input type="text3" placeholder={format!("Property {} name", prop_index+1)} value={self.document_types[doc_index].properties[prop_index].name.clone()} oninput={ctx.link().callback(move |e: InputEvent| Msg::UpdatePropertyName(doc_index, prop_index, e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().value()))} /></td>
                    <td>
                        <select value={selected_data_type.clone()} onchange={ctx.link().callback(move |e: Event| Msg::UpdatePropertyType(doc_index, prop_index, match e.target_dyn_into::<HtmlSelectElement>().unwrap().value().as_str() {
                            "String" => String::from("String"),
                            "Integer" => String::from("Integer"),
                            "Array" => String::from("Array"),
                            "Object" => String::from("Object"),
                            "Number" => String::from("Number"),
                            "Boolean" => String::from("Boolean"),
                            _ => panic!("Invalid data type selected"),
                        }))}>
                            {for data_type_options.iter().map(|option| html! {
                                <option value={String::from(*option)} selected={&String::from(*option)==&selected_data_type}>{String::from(*option)}</option>
                            })}
                        </select>
                    </td>
                    <td><input type="checkbox" checked={self.document_types[doc_index].properties[prop_index].required} onchange={ctx.link().callback(move |e: Event| Msg::UpdatePropertyRequired(doc_index, prop_index, e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().checked()))} /></td>
                    <td><button class="button" onclick={ctx.link().callback(move |_| Msg::RemoveProperty(doc_index, prop_index))}>{"Remove"}</button></td>
                </tr>
                <p><b>{if selected_data_type != String::from("Object") { "Optional property parameters:" } else {""}}</b></p>
                <tr>
                    <td colspan="4">
                        {additional_properties}
                        <tr>
                            <td><label>{"Description: "}</label></td>
                            <td><input type="text3" value={self.document_types[doc_index].properties[prop_index].description.clone()} oninput={ctx.link().callback(move |e: InputEvent| Msg::UpdatePropertyDescription(doc_index, prop_index, e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().value()))} /></td>
                        </tr>
                        <tr>
                            <td><label>{"Comment: "}</label></td>
                            <td><input type="text3" value={self.document_types[doc_index].properties[prop_index].comment.clone()} oninput={ctx.link().callback(move |e: InputEvent| Msg::UpdatePropertyComment(doc_index, prop_index, e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().value()))} /></td>
                        </tr>
                        <p></p>
                    </td>
                </tr>
            </>
        }
    }

    fn render_additional_properties(&self, data_type: &String, doc_index: usize, prop_index: usize, ctx: &yew::Context<Self>) -> Html {
        let property = &self.document_types[doc_index].properties[prop_index];
        match data_type.as_str() {
            "String" => html! {
                <>
                <tr>
                    <td><label>{"Min length: "}</label></td>
                    <td><input type="number" value={property.min_length.unwrap_or(0).to_string()} oninput={ctx.link().callback(move |e: InputEvent| Msg::UpdateStringPropertyMinLength(doc_index, prop_index, e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().value_as_number() as u32))} /></td>
                </tr>
                <tr>
                    <td><label>{"Max length: "}</label></td>
                    <td><input type="number" value={property.max_length.unwrap_or(0).to_string()} oninput={ctx.link().callback(move |e: InputEvent| Msg::UpdateStringPropertyMaxLength(doc_index, prop_index, e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().value_as_number() as u32))} /></td>
                </tr>
                <tr>
                    <td><label>{"RE2 pattern: "}</label></td>
                    <td><input type="text3" value={property.pattern.clone().unwrap_or_default()} oninput={ctx.link().callback(move |e: InputEvent| Msg::UpdateStringPropertyPattern(doc_index, prop_index, e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().value()))} /></td>
                </tr>
                <tr>
                    <td><label>{"Format: "}</label></td>
                    <td><input type="text3" value={property.format.clone().unwrap_or_default()} oninput={ctx.link().callback(move |e: InputEvent| Msg::UpdateStringPropertyFormat(doc_index, prop_index, e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().value()))} /></td>
                </tr>
                </>
            },
            "Integer" => html! {
                <>
                <tr>
                    <td><label>{"Minimum: "}</label></td>
                    <td><input type="number" value={property.minimum.unwrap_or(0).to_string()} oninput={ctx.link().callback(move |e: InputEvent| Msg::UpdateIntegerPropertyMinimum(doc_index, prop_index, e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().value_as_number() as i32))} /></td>
                </tr>
                <tr>
                    <td><label>{"Maximum: "}</label></td>
                    <td><input type="number" value={property.maximum.unwrap_or(0).to_string()} oninput={ctx.link().callback(move |e: InputEvent| Msg::UpdateIntegerPropertyMaximum(doc_index, prop_index, e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().value_as_number() as i32))} /></td>
                </tr>
                </>
            },
            "Array" => html! {
                <>
                <tr>
                    <td><label>{"Byte array: "}</label></td>
                    <td><input type="checkbox" checked={property.byte_array.unwrap_or(false)} onchange={ctx.link().callback(move |e: Event| Msg::UpdateArrayPropertyByteArray(doc_index, prop_index, e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().checked()))} /></td>
                </tr>
                <tr>
                    <td><label>{"Min items: "}</label></td>
                    <td><input type="number" oninput={ctx.link().callback(move |e: InputEvent| Msg::UpdateArrayPropertyMinItems(doc_index, prop_index, e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().value_as_number() as u32))} /></td>
                </tr>
                <tr>
                    <td><label>{"Max items: "}</label></td>
                    <td><input type="number" oninput={ctx.link().callback(move |e: InputEvent| Msg::UpdateArrayPropertyMaxItems(doc_index, prop_index, e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().value_as_number() as u32))} /></td>
                </tr>
                </>
            },
            "Object" => html! {
                <>
                <tr>
                    <td colspan="4">
                    {for self.document_types[doc_index].properties[prop_index].properties.as_ref().unwrap_or(&Box::new(Vec::new())).iter().enumerate().map(|(i, _)| self.view_recursive_property(doc_index, prop_index, i, ctx))}
                    </td>
                </tr>
                <tr>
                    <td><button class="button" onclick={ctx.link().callback(move |_| Msg::AddRecProperty(doc_index, prop_index))}>{"Add Inner Property"}</button></td>
                </tr>
                {self.rec_render_additional_properties(&data_type, doc_index, prop_index, 0, ctx)}
                <p><b>{"Optional property parameters:"}</b></p>
                <tr>
                    <td><label>{"Min properties: "}</label></td>
                    <td><input type="number" value={property.min_properties.unwrap_or(0).to_string()} oninput={ctx.link().callback(move |e: InputEvent| Msg::UpdateObjectPropertyMinProperties(doc_index, prop_index, e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().value_as_number() as u32))} /></td>
                </tr>
                <tr>
                    <td><label>{"Max properties: "}</label></td>
                    <td><input type="number" value={property.max_properties.unwrap_or(0).to_string()} oninput={ctx.link().callback(move |e: InputEvent| Msg::UpdateObjectPropertyMaxProperties(doc_index, prop_index, e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().value_as_number() as u32))} /></td>
                </tr>
                </>
            },
            "Number" => html! {
                <>
                </>
            },
            "Boolean" => html! {
                <>
                </>
            },
            _ => html! {},
        }
    }

    fn view_recursive_property(&self, doc_index: usize, prop_index: usize, recursive_prop_index: usize, ctx: &yew::Context<Self>) -> Html {
        let data_type_options = vec!["String", "Integer", "Array", "Object", "Number", "Boolean"];
        let selected_data_type = match &self.document_types[doc_index].properties[prop_index].properties.clone() {
            Some(properties) => match properties.get(recursive_prop_index) {
                Some(property) => match property.data_type {
                    DataType::String => String::from("String"),
                    DataType::Integer => String::from("Integer"),
                    DataType::Array => String::from("Array"),
                    DataType::Object => String::from("Object"),
                    DataType::Number => String::from("Number"),
                    DataType::Boolean => String::from("Boolean"),
                },
                None => return html! {<>{"oops1"}</>},
            },
            None => return html! {<>{"oops2"}</>},
        };
    
        html! {
            <>
                <tr>
                    <th>{"Name"}</th>
                    <th>{"Type"}</th>
                    <th>{"Required"}</th>
                </tr>
                <tr>
                    <td>
                        <input type="text3" placeholder={format!("Inner property {} name", recursive_prop_index+1)} value={match &self.document_types[doc_index].properties[prop_index].properties {
                            Some(properties) => properties.get(recursive_prop_index).map(|property| property.name.clone()).unwrap_or_default(),
                            None => String::new(),
                        }} oninput={ctx.link().callback(move |e: InputEvent| Msg::UpdateRecPropertyName(doc_index, prop_index, recursive_prop_index, e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().value()))} />
                    </td>
                    <td>
                        <select value={selected_data_type.clone()} onchange={ctx.link().callback(move |e: Event| Msg::UpdateRecPropertyType(doc_index, prop_index, recursive_prop_index, match e.target_dyn_into::<HtmlSelectElement>().unwrap().value().as_str() {
                            "String" => String::from("String"),
                            "Integer" => String::from("Integer"),
                            "Array" => String::from("Array"),
                            "Object" => String::from("Object"),
                            "Number" => String::from("Number"),
                            "Boolean" => String::from("Boolean"),
                            _ => panic!("Invalid data type selected"),
                        }))}>
                            {for data_type_options.iter().map(|option| html! {
                                <option value={String::from(*option)} selected={&String::from(*option)==&selected_data_type}>{String::from(*option)}</option>
                            })}
                        </select>
                    </td>
                    <td>
                        <input type="checkbox" checked={match &self.document_types[doc_index].properties[prop_index].properties {
                            Some(properties) => properties.get(recursive_prop_index).map(|property| property.required).unwrap_or(false),
                            None => false,
                        }} onchange={ctx.link().callback(move |e: Event| Msg::UpdateRecPropertyRequired(doc_index, prop_index, recursive_prop_index, e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().checked()))} />
                    </td>
                    <td>
                        <button class="button" onclick={ctx.link().callback(move |_| Msg::RemoveRecProperty(doc_index, prop_index, recursive_prop_index))}>{"Remove"}</button>
                    </td>
                </tr>
                <p><b>{"Optional property parameters:"}</b></p>
                <tr>
                    <td colspan="4">
                        <table>
                            {self.rec_render_additional_properties(&selected_data_type, doc_index, prop_index, recursive_prop_index, ctx)}
                            <tr>
                                <td><label>{"Description: "}</label></td>
                                <td><input type="text3" value={if let Some(properties) = &self.document_types.get(doc_index).and_then(|doc| doc.properties.get(prop_index).and_then(|prop| prop.properties.clone())) {
                                    properties.get(recursive_prop_index).and_then(|prop| prop.description.clone()).unwrap_or_default()
                                } else {
                                    "".to_string()
                                }} oninput={ctx.link().callback(move |e: InputEvent| Msg::UpdateRecPropertyDescription(doc_index, prop_index, recursive_prop_index, e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().value()))} /></td>
                            </tr>
                            <tr>
                                <td><label>{"Comment: "}</label></td><td><input type="text3" value={if let Some(properties) = &self.document_types.get(doc_index).and_then(|doc| doc.properties.get(prop_index).and_then(|prop| prop.properties.clone())) {
                                    properties.get(recursive_prop_index).and_then(|prop| prop.comment.clone()).unwrap_or_default()
                                } else {
                                    "".to_string()
                                }} oninput={ctx.link().callback(move |e: InputEvent| Msg::UpdateRecPropertyComment(doc_index, prop_index, recursive_prop_index, e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().value()))} /></td>
                            </tr>
                            <p></p>
                        </table>
                    </td>
                </tr>
            </>
        }
    }

    fn rec_render_additional_properties(&self, data_type: &String, doc_index: usize, prop_index: usize, recursive_prop_index: usize, ctx: &yew::Context<Self>) -> Html {
        match data_type.as_str() {
            "String" => {
                if let Some(_props) = &self.document_types[doc_index].properties[prop_index].properties {
                    html! {
                        <>
                        <tr>
                            <td><label>{"Min length: "}</label></td>
                            <td><input type="number" oninput={ctx.link().callback(move |e: InputEvent| Msg::UpdateStringRecPropertyMinLength(doc_index, prop_index, recursive_prop_index, e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().value_as_number() as u32))} /></td>
                        </tr>
                        <tr>
                            <td><label>{"Max length: "}</label></td>
                            <td><input type="number" oninput={ctx.link().callback(move |e: InputEvent| Msg::UpdateStringRecPropertyMaxLength(doc_index, prop_index, recursive_prop_index, e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().value_as_number() as u32))} /></td>
                        </tr>
                        <tr>
                            <td><label>{"RE2 pattern: "}</label></td>
                            <td><input type="text3" oninput={ctx.link().callback(move |e: InputEvent| Msg::UpdateStringRecPropertyPattern(doc_index, prop_index, recursive_prop_index, e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().value()))} /></td>
                        </tr>
                        <tr>
                            <td><label>{"Format: "}</label></td>
                            <td><input type="text3" oninput={ctx.link().callback(move |e: InputEvent| Msg::UpdateStringRecPropertyFormat(doc_index, prop_index, recursive_prop_index, e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().value()))} /></td>
                        </tr>
                        </>
                    }
                } else {
                    html! {}
                }
            },
            "Integer" => {
                if let Some(_props) = &self.document_types[doc_index].properties[prop_index].properties {
                    html! {
                        <>
                        <tr>
                            <td><label>{"Minimum: "}</label></td>
                            <td><input type="number" oninput={ctx.link().callback(move |e: InputEvent| Msg::UpdateIntegerRecPropertyMinimum(doc_index, prop_index, recursive_prop_index, e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().value_as_number() as i32))} /></td>
                        </tr>
                        <tr>
                            <td><label>{"Maximum: "}</label></td>
                            <td><input type="number" oninput={ctx.link().callback(move |e: InputEvent| Msg::UpdateIntegerRecPropertyMaximum(doc_index, prop_index, recursive_prop_index, e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().value_as_number() as i32))} /></td>
                        </tr>
                        </>
                    }
                } else {
                    html! {}
                }
            },
            "Array" => {
                let properties = self.document_types[doc_index].properties.get(prop_index).and_then(|p| p.properties.as_ref());
                let byte_array = properties.and_then(|p| p.get(recursive_prop_index)).and_then(|p| p.byte_array);
                let max_items = properties.and_then(|p| p.get(recursive_prop_index)).and_then(|p| p.max_items);
                let min_items = properties.and_then(|p| p.get(recursive_prop_index)).and_then(|p| p.min_items);
            
                html! {
                    <>
                    <tr>
                        <td><label>{"Byte array: "}</label></td>
                        <td><input type="checkbox" checked={byte_array.unwrap_or(false)} onchange={ctx.link().callback(move |e: Event| Msg::UpdateArrayRecPropertyByteArray(doc_index, prop_index, recursive_prop_index, e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().checked()))} /></td>
                    </tr>
                    <tr>
                        <td><label>{"Min items: "}</label></td>
                        <td><input type="number" oninput={ctx.link().callback(move |e: InputEvent| {
                            let value = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().value_as_number();
                            let value = match value {
                                v if v.is_finite() => Some(v as u32),
                                _ => None,
                            };
                            Msg::UpdateArrayRecPropertyMinItems(doc_index, prop_index, recursive_prop_index, value.unwrap_or(0))
                        })} value={min_items.map(|n| n.to_string()).unwrap_or_default().to_owned()} /></td>
                    </tr>
                    <tr>
                        <td><label>{"Max items: "}</label></td>
                        <td><input type="number" oninput={ctx.link().callback(move |e: InputEvent| {
                            let value = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().value_as_number();
                            let value = match value {
                                v if v.is_finite() => Some(v as u32),
                                _ => None,
                            };
                            Msg::UpdateArrayRecPropertyMaxItems(doc_index, prop_index, recursive_prop_index, value.unwrap_or(0))
                        })} value={max_items.map(|n| n.to_string()).unwrap_or_default().to_owned()} /></td>
                    </tr>
                    </>
                }
            },            
            "Object" => {
                let properties = self.document_types[doc_index].properties.get(prop_index).and_then(|p| p.properties.as_ref());
                let min_props = properties.and_then(|p| p.get(recursive_prop_index)).and_then(|p| p.min_properties);
                let max_props = properties.and_then(|p| p.get(recursive_prop_index)).and_then(|p| p.max_properties);
            
                html! {
                    <>
                    //<tr>
                    //    <td><label>{"Min properties: "}</label></td>
                    //    <td><input type="number" oninput={ctx.link().callback(move |e: InputEvent| {
                    //        let value = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().value_as_number();
                    //        let value = match value {
                    //            v if v.is_finite() => Some(v as u32),
                    //            _ => None,
                    //        };
                    //        Msg::UpdateObjectRecPropertyMinProperties(doc_index, prop_index, recursive_prop_index, value.unwrap_or(0))
                    //    })} value={min_props.map(|n| n.to_string()).unwrap_or_default().to_owned()} /></td>
                    //</tr>
                    //<tr>
                    //    <td><label>{"Max properties: "}</label></td>
                    //    <td><input type="number" oninput={ctx.link().callback(move |e: InputEvent| {
                    //        let value = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().value_as_number();
                    //        let value = match value {
                    //            v if v.is_finite() => Some(v as u32),
                    //            _ => None,
                    //        };
                    //        Msg::UpdateObjectRecPropertyMaxProperties(doc_index, prop_index, recursive_prop_index, value.unwrap_or(0))
                    //    })} value={max_props.map(|n| n.to_string()).unwrap_or_default().to_owned()} /></td>
                    //</tr>
                    </>
                }
            },
            "Number" => html! {
                <>
                </>
            },
            "Boolean" => html! {
                <>
                </>
            },
            _ => html! {},
        }
    }

    fn view_index(&self, doc_index: usize, index_index: usize, ctx: &yew::Context<Self>) -> Html {
        html! {
            <>
            <tr>
                <th>{"Name"}</th>
                <th>{"Unique"}</th>
                <th>{""}</th>
            </tr>
            <tr>
                <td><input type="text3" placeholder={format!("Index {} name", index_index+1)} value={self.document_types[doc_index].indices[index_index].name.clone()} oninput={ctx.link().callback(move |e: InputEvent| Msg::UpdateIndexName(doc_index, index_index, e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().value()))} /></td>
                <td><input type="checkbox" checked={self.document_types[doc_index].indices[index_index].unique} onchange={ctx.link().callback(move |e: Event| Msg::UpdateIndexUnique(doc_index, index_index, e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().checked()))} /></td>
                <td><button class="button" onclick={ctx.link().callback(move |_| Msg::RemoveIndex(doc_index, index_index))}>{"Remove"}</button></td>
            </tr>
            <tr>
                <td colspan="3">
                    <table>
                        <tbody>
                            <p><b>{"Index properties:"}</b></p>
                            <div>{for (0..self.document_types[doc_index].indices[index_index].properties.len()).map(|i| self.view_index_properties(doc_index, index_index, i, ctx))}</div>
                        </tbody>
                    </table>
                </td>
            </tr>
            <p></p>
            <tr>
                <td colspan="2"><button class="button" onclick={ctx.link().callback(move |_| Msg::AddIndexProperty(doc_index, index_index))}>{"Add index property"}</button></td>
            </tr>
            <p></p>
            </>
        }
    }    

    fn view_index_properties(&self, doc_index: usize, index_index: usize, prop_index: usize, ctx: &yew::Context<Self>) -> Html {
        let sorting_options = vec!["Ascending", "Descending"];
        html!(
            <tr>
                <td></td>
                <td><label>{format!("Property {}: ", prop_index+1)}</label><input type="text3" value={self.document_types[doc_index].indices[index_index].properties[prop_index].0.clone()} oninput={ctx.link().callback(move |e: InputEvent| Msg::UpdateIndexProperty(doc_index, index_index, prop_index, e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().value()))} /></td>
                <td><select value={sorting_options[0]} onchange={ctx.link().callback(move |e: Event| Msg::UpdateIndexSorting(doc_index, index_index, prop_index, match e.target_dyn_into::<HtmlSelectElement>().unwrap().value().as_str() {
                    "Ascending" => String::from("asc"),
                    "Descending" => String::from("desc"),
                    _ => panic!("Invalid data type selected"),
                }))}>
                    {for sorting_options.iter().map(|option| html! {
                        <option value={String::from(*option)} selected={&String::from(*option)==sorting_options[0]}>{String::from(*option)}</option>
                    })}
                </select></td>
            </tr>
        )
    }

    fn generate_json_object(&mut self) -> Vec<String> {
        println!("generate_json_object: start");
        let mut json_arr = Vec::new();
        for doc_type in &mut self.document_types {
            println!("generate_json_object: iterating document types");
            let mut props_map = Map::new();
            for prop in &mut doc_type.properties {
                println!("generate_json_object: iterating properties");
                let mut prop_obj = Map::new();
                prop_obj.insert("type".to_owned(), json!(match prop.data_type {
                    DataType::String => "string",
                    DataType::Integer => "integer",
                    DataType::Array => "array",
                    DataType::Object => "object",
                    DataType::Number => "number",
                    DataType::Boolean => "bool",
                }));
                if prop.description.as_ref().map(|c| c.len()).unwrap_or(0) > 0 {
                    prop_obj.insert("description".to_owned(), json!(prop.description));
                }
                if prop.min_length.as_ref().map(|c| *c).unwrap_or(0) > 0 {
                    prop_obj.insert("minLength".to_owned(), json!(prop.min_length));
                }
                if prop.max_length.as_ref().map(|c| *c).unwrap_or(0) > 0 {
                    prop_obj.insert("maxLength".to_owned(), json!(prop.max_length));
                }
                if prop.pattern.as_ref().map(|c| c.len()).unwrap_or(0) > 0 {
                    prop_obj.insert("pattern".to_owned(), json!(prop.pattern));
                }
                if prop.format.as_ref().map(|c| c.len()).unwrap_or(0) > 0 {
                    prop_obj.insert("format".to_owned(), json!(prop.format));
                }
                if prop.minimum.as_ref().map(|c| *c).unwrap_or(0) > 0 {
                    prop_obj.insert("minimum".to_owned(), json!(prop.minimum));
                }
                if prop.maximum.as_ref().map(|c| *c).unwrap_or(0) > 0 {
                    prop_obj.insert("maximum".to_owned(), json!(prop.maximum));
                }
                if let Some(byte_array) = prop.byte_array {
                    prop_obj.insert("byteArray".to_owned(), json!(byte_array));
                }
                if prop.min_items.as_ref().map(|c| *c).unwrap_or(0) > 0 {
                    prop_obj.insert("minItems".to_owned(), json!(prop.min_items));
                }
                if prop.max_items.as_ref().map(|c| *c).unwrap_or(0) > 0 {
                    prop_obj.insert("maxItems".to_owned(), json!(prop.max_items));
                }
                if prop.data_type == DataType::Object {
                    let rec_props_map = Self::generate_nested_properties(prop);
                    prop_obj.insert("properties".to_owned(), json!(rec_props_map));
                    }
                if prop.min_properties.as_ref().map(|c| *c).unwrap_or(0) > 0 {
                    prop_obj.insert("minProperties".to_owned(), json!(prop.min_properties));
                }
                if prop.max_properties.as_ref().map(|c| *c).unwrap_or(0) > 0 {
                    prop_obj.insert("maxProperties".to_owned(), json!(prop.max_properties));
                }
                if prop.rec_required.as_ref().map(|c| c.len()).unwrap_or_default() > 0 {
                    prop_obj.insert("required".to_owned(), json!(prop.rec_required));
                }
                if prop.data_type == DataType::Object {
                    prop_obj.insert("additionalProperties".to_owned(), json!(false));
                }
                if prop.comment.as_ref().map(|c| c.len()).unwrap_or(0) > 0 {
                    prop_obj.insert("$comment".to_owned(), json!(prop.comment));
                }
                props_map.insert(prop.name.clone(), json!(prop_obj));
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
                if index.unique {
                    let index_obj = json!({
                        "name": index.name,
                        "properties": index.properties.iter().map(|inner_tuple| {
                            let mut inner_obj = Map::new();
                            inner_obj.insert(inner_tuple.0.clone(), json!(inner_tuple.1));
                            json!(inner_obj)
                        }).collect::<Vec<_>>(),
                        "unique": index.unique,
                    });
                    indices_arr.push(index_obj);
                } else {
                    let index_obj = json!({
                        "name": index.name,
                        "properties": index.properties.iter().map(|inner_tuple| {
                            let mut inner_obj = Map::new();
                            inner_obj.insert(inner_tuple.0.clone(), json!(inner_tuple.1));
                            json!(inner_obj)
                        }).collect::<Vec<_>>(),
                    });
                    indices_arr.push(index_obj);
                }
            }
            let mut doc_obj = Map::new();
            doc_obj.insert("type".to_owned(), json!("object"));
            doc_obj.insert("properties".to_owned(), json!(props_map));
            if !doc_type.indices.is_empty() {
                doc_obj.insert("indices".to_owned(), json!(indices_arr));
            }
            if !doc_type.required.is_empty() {
                doc_obj.insert("required".to_owned(), json!(doc_type.required));
            }
            doc_obj.insert("additionalProperties".to_owned(), json!(false));
            if doc_type.comment.len() > 0 {
                doc_obj.insert("$comment".to_owned(), json!(doc_type.comment));
            }
            let final_doc_obj = json!({
                doc_type.name.clone(): doc_obj
            });
            let formatted_doc_obj = &final_doc_obj.to_string()[1..final_doc_obj.to_string().len()-1];
            json_arr.push(formatted_doc_obj.to_string());
        }
        println!("generate_json_object: end");
        json_arr
    }    

    fn generate_nested_properties(prop: &mut Property) -> Map<String, Value> {
        let mut rec_props_map = Map::new();
        if let Some(nested_props) = &mut prop.properties {
            for rec_prop in nested_props.iter_mut() {
                let mut rec_prop_obj = Map::new();
                rec_prop_obj.insert("type".to_owned(), json!(match rec_prop.data_type {
                    DataType::String => "string",
                    DataType::Integer => "integer",
                    DataType::Array => "array",
                    DataType::Object => "object",
                    DataType::Number => "number",
                    DataType::Boolean => "bool",
                }));
                if rec_prop.description.as_ref().map(|c| c.len()).unwrap_or(0) > 0 {
                    rec_prop_obj.insert("description".to_owned(), json!(rec_prop.description));
                }
                if rec_prop.min_length.as_ref().map(|c| *c).unwrap_or(0) > 0 {
                    rec_prop_obj.insert("minLength".to_owned(), json!(rec_prop.min_length));
                }
                if rec_prop.max_length.as_ref().map(|c| *c).unwrap_or(0) > 0 {
                    rec_prop_obj.insert("maxLength".to_owned(), json!(rec_prop.max_length));
                }
                if rec_prop.pattern.as_ref().map(|c| c.len()).unwrap_or(0) > 0 {
                    rec_prop_obj.insert("pattern".to_owned(), json!(rec_prop.pattern));
                }
                if rec_prop.format.as_ref().map(|c| c.len()).unwrap_or(0) > 0 {
                    rec_prop_obj.insert("format".to_owned(), json!(rec_prop.format));
                }
                if rec_prop.minimum.as_ref().map(|c| *c).unwrap_or(0) > 0 {
                    rec_prop_obj.insert("minimum".to_owned(), json!(rec_prop.minimum));
                }
                if rec_prop.maximum.as_ref().map(|c| *c).unwrap_or(0) > 0 {
                    rec_prop_obj.insert("maximum".to_owned(), json!(rec_prop.maximum));
                }
                if let Some(byte_array) = rec_prop.byte_array {
                    rec_prop_obj.insert("byteArray".to_owned(), json!(byte_array));
                }
                if rec_prop.min_items.as_ref().map(|c| *c).unwrap_or(0) > 0 {
                    rec_prop_obj.insert("minItems".to_owned(), json!(rec_prop.min_items));
                }
                if rec_prop.max_items.as_ref().map(|c| *c).unwrap_or(0) > 0 {
                    rec_prop_obj.insert("maxItems".to_owned(), json!(rec_prop.max_items));
                }
                if rec_prop.data_type == DataType::Object {
                    rec_prop_obj.insert("properties".to_owned(), json!({}));
                }
                if rec_prop.min_properties.as_ref().map(|c| *c).unwrap_or(0) > 0 {
                    rec_prop_obj.insert("minProperties".to_owned(), json!(rec_prop.min_properties));
                }
                if rec_prop.max_properties.as_ref().map(|c| *c).unwrap_or(0) > 0 {
                    rec_prop_obj.insert("maxProperties".to_owned(), json!(rec_prop.max_properties));
                }
                if rec_prop.data_type == DataType::Object {
                    rec_prop_obj.insert("additionalProperties".to_owned(), json!(false));
                }
                if rec_prop.comment.as_ref().map(|c| c.len()).unwrap_or(0) > 0 {
                    rec_prop_obj.insert("$comment".to_owned(), json!(rec_prop.comment));
                }
                rec_props_map.insert(rec_prop.name.clone(), json!(rec_prop_obj));
                if rec_prop.required {
                    if !prop.rec_required.as_ref().cloned().unwrap_or_default().contains(&rec_prop.name) {
                        prop.rec_required.get_or_insert_with(Vec::new).push(rec_prop.name.clone());
                        console::log_1(&format!("rec_prop.name inserted into prop.rec_required: {:?}", prop.rec_required).into());
                    }
                } else {
                    if prop.rec_required.as_ref().map_or(false, |req| req.contains(&rec_prop.name)) {
                        prop.rec_required.as_mut().map(|v| v.retain(|x| x != &rec_prop.name));
                    }
                }
                rec_props_map.insert(rec_prop.name.clone(), json!(rec_prop_obj));
            }
        }
        rec_props_map
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        let mut default_document_type = DocumentType::default();
        default_document_type.properties.push(Property::default());
        Self {
            document_types: vec![default_document_type],
            json_object: vec![],
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // General
            Msg::AddDocumentType => {
                let mut new_document_type = DocumentType::default();
                new_document_type.properties.push(Property::default());
                self.document_types.push(new_document_type);
                true
            }
            Msg::AddProperty(index) => {
                self.document_types[index].properties.push(Default::default());
                true
            }
            Msg::AddIndex(index) => {
                self.document_types[index].indices.push(Index {
                    name: String::new(),
                    unique: false,
                    properties: vec![IndexProperties::default()],
                });
                true
            }
            Msg::RemoveDocumentType(index) => {
                self.document_types.remove(index);
                true
            }
            Msg::RemoveProperty(doc_index, prop_index) => {
                let name = self.document_types[doc_index].properties[prop_index].name.clone();
                let required = &mut self.document_types[doc_index].required;
                if let Some(index) = required.iter().position(|x| x == &name) {
                    required.remove(index);
                }
                self.document_types[doc_index].properties.remove(prop_index);
                true
            }
            Msg::RemoveIndex(doc_index, index_index) => {
                self.document_types[doc_index].indices.remove(index_index);
                true
            }
            Msg::AddIndexProperty(doc_index, index_index) => {
                self.document_types[doc_index].indices[index_index].properties.push(Default::default());
                true
            }
            Msg::Submit => {
                self.json_object = Some(self.generate_json_object()).unwrap();
                true
            }
            Msg::UpdateName(index, name) => {
                self.document_types[index].name = name;
                true
            }
            Msg::UpdateComment(index, comment) => {
                self.document_types[index].comment = comment;
                true
            }
            Msg::UpdatePropertyName(doc_index, prop_index, name) => {
                self.document_types[doc_index].properties[prop_index].name = name;
                true
            }
            Msg::UpdateIndexName(doc_index, index_index, name) => {
                self.document_types[doc_index].indices[index_index].name = name;
                true
            }
            Msg::UpdateIndexProperty(doc_index, index_index, prop_index, prop) => {
                self.document_types[doc_index].indices[index_index].properties[prop_index].0 = prop;
                true
            }
            Msg::UpdateIndexSorting(doc_index, index_index, prop_index, sorting) => {
                self.document_types[doc_index].indices[index_index].properties[prop_index].1 = sorting;
                true
            }
            Msg::UpdatePropertyType(doc_index, prop_index, data_type) => {
                let data_type = match data_type.as_str() {
                    "String" => DataType::String,
                    "Integer" => DataType::Integer,
                    "Array" => DataType::Array,
                    "Object" => DataType::Object,
                    "Number" => DataType::Number,
                    "Boolean" => DataType::Boolean,
                    _ => unreachable!(),
                };
                self.document_types[doc_index].properties[prop_index].data_type = data_type;
                true
            }
            Msg::UpdateIndexUnique(doc_index, index_index, unique) => {
                self.document_types[doc_index].indices[index_index].unique = unique;
                true
            }
            Msg::UpdatePropertyRequired(doc_index, prop_index, required) => {
                self.document_types[doc_index].properties[prop_index].required = required;
                true
            }
            Msg::UpdatePropertyDescription(doc_index, prop_index, description) => {
                self.document_types[doc_index].properties[prop_index].description = Some(description);
                true
            }
            Msg::UpdatePropertyComment(doc_index, prop_index, comment) => {
                self.document_types[doc_index].properties[prop_index].comment = Some(comment);
                true
            }

            // Optional property parameters
            Msg::UpdateStringPropertyMinLength(doc_index, prop_index, min_length) => {
                self.document_types[doc_index].properties[prop_index].min_length = Some(min_length);
                true
            }
            Msg::UpdateStringPropertyMaxLength(doc_index, prop_index, max_length) => {
                self.document_types[doc_index].properties[prop_index].max_length = Some(max_length);
                true
            }
            Msg::UpdateStringPropertyPattern(doc_index, prop_index, pattern) => {
                self.document_types[doc_index].properties[prop_index].pattern = Some(pattern);
                true
            }
            Msg::UpdateStringPropertyFormat(doc_index, prop_index, format) => {
                self.document_types[doc_index].properties[prop_index].format = Some(format);
                true
            }
            Msg::UpdateIntegerPropertyMinimum(doc_index, prop_index, minimum) => {
                self.document_types[doc_index].properties[prop_index].minimum = Some(minimum);
                true
            }
            Msg::UpdateIntegerPropertyMaximum(doc_index, prop_index, maximum) => {
                self.document_types[doc_index].properties[prop_index].maximum = Some(maximum);
                true
            }
            Msg::UpdateArrayPropertyByteArray(doc_index, prop_index, byte_array) => {
                self.document_types[doc_index].properties[prop_index].byte_array = Some(byte_array);
                true
            }
            Msg::UpdateArrayPropertyMinItems(doc_index, prop_index, min_items) => {
                self.document_types[doc_index].properties[prop_index].min_items = Some(min_items);
                true
            }
            Msg::UpdateArrayPropertyMaxItems(doc_index, prop_index, max_items) => {
                self.document_types[doc_index].properties[prop_index].max_items = Some(max_items);
                true
            }
            Msg::UpdateObjectPropertyMinProperties(doc_index, prop_index, min_properties) => {
                self.document_types[doc_index].properties[prop_index].min_properties = Some(min_properties);
                true
            }
            Msg::UpdateObjectPropertyMaxProperties(doc_index, prop_index, max_properties) => {
                self.document_types[doc_index].properties[prop_index].max_properties = Some(max_properties);
                true
            }

            // Recursive properties
            Msg::AddRecProperty(doc_index, prop_index) => {
                let property = Property {
                    name: String::new(),
                    data_type: DataType::String,
                    required: false,
                    rec_required: Some(Vec::new()),
                    description: None,
                    comment: None,
                    properties: None,
                    additional_properties: None,
                    byte_array: None,
                    format: None,
                    pattern: None,
                    min_length: None,
                    max_length: None,
                    minimum: None,
                    maximum: None,
                    min_items: None,
                    max_items: None,
                    min_properties: None,
                    max_properties: None,
                };
    
                let document_type = self.document_types.get_mut(doc_index);
                if let Some(document_type) = document_type {
                    if let Some(properties) = document_type.properties.get_mut(prop_index).and_then(|prop| prop.properties.as_mut()) {
                        properties.push(property);
                    } else {
                        document_type.properties[prop_index].properties = Some(Box::new(vec![property]));
                    }
                }
                true
            }  
            Msg::RemoveRecProperty(doc_index, prop_index, rec_prop_index) => {
                if let Some(property_vec) = self.document_types[doc_index].properties[prop_index].properties.as_mut() {
                    property_vec.remove(rec_prop_index);
                }
                true
            }
            Msg::UpdateRecPropertyName(doc_index, prop_index, rec_prop_index, name) => {
                if let Some(property_vec) = self.document_types[doc_index].properties[prop_index].properties.as_mut() {
                    property_vec[rec_prop_index].name = name;
                }
                true
            }
            Msg::UpdateRecPropertyType(doc_index, prop_index, rec_prop_index, data_type) => {
                let data_type = match data_type.as_str() {
                    "String" => DataType::String,
                    "Integer" => DataType::Integer,
                    "Array" => DataType::Array,
                    "Object" => DataType::Object,
                    "Number" => DataType::Number,
                    "Boolean" => DataType::Boolean,
                    _ => unreachable!(),
                };
                if let Some(property_vec) = self.document_types[doc_index].properties[prop_index].properties.as_mut() {
                    property_vec[rec_prop_index].data_type = data_type;
                }
                true
            }
            Msg::UpdateRecPropertyRequired(doc_index, prop_index, rec_prop_index, required) => {
                if let Some(property_vec) = self.document_types[doc_index].properties[prop_index].properties.as_mut() {
                    property_vec[rec_prop_index].required = required;
                }
                true
            }
            Msg::UpdateRecPropertyDescription(doc_index, prop_index, rec_prop_index, description) => {
                if let Some(property_vec) = self.document_types[doc_index].properties[prop_index].properties.as_mut() {
                    property_vec[rec_prop_index].description = Some(description);
                }
                true
            }
            Msg::UpdateRecPropertyComment(doc_index, prop_index, rec_prop_index, comment) => {
                if let Some(property_vec) = self.document_types[doc_index].properties[prop_index].properties.as_mut() {
                    property_vec[rec_prop_index].comment = Some(comment);
                }
                true
            }
            Msg::UpdateStringRecPropertyMinLength(doc_index, prop_index, rec_prop_index, min_length) => {
                if let Some(property_vec) = self.document_types[doc_index].properties[prop_index].properties.as_mut() {
                    property_vec[rec_prop_index].min_length = Some(min_length);
                }
                true
            }
            Msg::UpdateStringRecPropertyMaxLength(doc_index, prop_index, rec_prop_index, max_length) => {
                if let Some(property_vec) = self.document_types[doc_index].properties[prop_index].properties.as_mut() {
                    property_vec[rec_prop_index].max_length = Some(max_length);
                }
                true
            }
            Msg::UpdateStringRecPropertyPattern(doc_index, prop_index, rec_prop_index, pattern) => {
                if let Some(property_vec) = self.document_types[doc_index].properties[prop_index].properties.as_mut() {
                    property_vec[rec_prop_index].pattern = Some(pattern);
                }
                true
            }
            Msg::UpdateStringRecPropertyFormat(doc_index, prop_index, rec_prop_index, format) => {
                if let Some(property_vec) = self.document_types[doc_index].properties[prop_index].properties.as_mut() {
                    property_vec[rec_prop_index].format = Some(format);
                }
                true
            }
            Msg::UpdateIntegerRecPropertyMaximum(doc_index, prop_index, rec_prop_index, maximum) => {
                if let Some(property_vec) = self.document_types[doc_index].properties[prop_index].properties.as_mut() {
                    property_vec[rec_prop_index].maximum = Some(maximum);
                }
                true
            }
            Msg::UpdateIntegerRecPropertyMinimum(doc_index, prop_index, rec_prop_index, minimum) => {
                if let Some(property_vec) = self.document_types[doc_index].properties[prop_index].properties.as_mut() {
                    property_vec[rec_prop_index].minimum = Some(minimum);
                }
                true
            }
            Msg::UpdateArrayRecPropertyByteArray(doc_index, prop_index, rec_prop_index, byte_array) => {
                if let Some(property_vec) = self.document_types[doc_index].properties[prop_index].properties.as_mut() {
                    property_vec[rec_prop_index].byte_array = Some(byte_array);
                }
                true
            }
            Msg::UpdateArrayRecPropertyMinItems(doc_index, prop_index, rec_prop_index, min_items) => {
                if let Some(property_vec) = self.document_types[doc_index].properties[prop_index].properties.as_mut() {
                    property_vec[rec_prop_index].min_items = Some(min_items);
                }
                true
            }
            Msg::UpdateArrayRecPropertyMaxItems(doc_index, prop_index, rec_prop_index, max_items) => {
                if let Some(property_vec) = self.document_types[doc_index].properties[prop_index].properties.as_mut() {
                    property_vec[rec_prop_index].max_items = Some(max_items);
                }
                true
            }
            Msg::UpdateObjectRecPropertyMinProperties(doc_index, prop_index, rec_prop_index, min_props) => {
                if let Some(property_vec) = self.document_types[doc_index].properties[prop_index].properties.as_mut() {
                    property_vec[rec_prop_index].min_properties = Some(min_props);
                }
                true
            }
            Msg::UpdateObjectRecPropertyMaxProperties(doc_index, prop_index, rec_prop_index, max_props) => {
                if let Some(property_vec) = self.document_types[doc_index].properties[prop_index].properties.as_mut() {
                    property_vec[rec_prop_index].max_properties = Some(max_props);
                }
                true
            }
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {        
        // html
        html! {
            <main class="home">
            <img class="logo" src="https://media.dash.org/wp-content/uploads/dash-logo.svg" alt="Dash logo" width="200" height="100" />
            <br/><br/>
            <h1 class="header">{"Data Contract Creator"}</h1>
            <body>
            <div class="column-left">

                // show input fields
                <p class="input-fields">{self.view_document_types(ctx)}</p>

                <div class="button-container">
                    // add input fields for another document type and add one to Self::document_types
                    <button class="button2" onclick={ctx.link().callback(|_| Msg::AddDocumentType)}>{"Add document type"}</button><br/>

                    // look at document_types and generate json object from it
                    <button class="button button-primary" onclick={ctx.link().callback(|_| Msg::Submit)}>{"Submit"}</button>
                </div>
                <div class="footnotes">
                    <h3>{"Notes"}</h3>
                    <p>{"For indexes on inner properties: use the format 'outerProperty.innerProperty'."}</p>
                </div>
            </div>
            <div class="column-right">
            
                // format and display json object
                <p class="output-container">
                    <h2>{"Contract"}</h2>
                    <h3>{if self.json_object.len() != 0 as usize {"With whitespace:"} else {""}}</h3>
                    <pre>
                    {if self.json_object.len() != 0 as usize {
                        let s = &self.json_object.join(",");
                        let new_s = format!("{{{}}}", s);
                        let json_obj: serde_json::Value = serde_json::from_str(&new_s).unwrap();
                        serde_json::to_string_pretty(&json_obj).unwrap()
                    } else { 
                        "".to_string()
                    }}
                    </pre>
                    <h3>{if self.json_object.len() != 0 as usize {"Without whitespace:"} else {""}}</h3>
                    <pre>
                    {if self.json_object.len() != 0 as usize {
                        let s = &self.json_object.join(",");
                        let new_s = format!("{{{}}}", s);
                        let json_obj: serde_json::Value = serde_json::from_str(&new_s).unwrap();
                        serde_json::to_string(&json_obj).unwrap()
                    } else { 
                        "".to_string()
                    }}
                    </pre>
                </p>

            </div>
            </body>
            </main>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Model>::new().render();
}

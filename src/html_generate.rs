use serde_json::value::{Map, Value as Json};
use std::error::Error;
use std::fs::File;
use handlebars::{
    to_json, Context, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError,
};

#[derive(Serialize)]
pub struct ResultHolder {
    id: String,
    response_time: u32,
    execution_time: u32,
    blocking_time: u32,
    preemption_time: u32,
}

fn format(h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> Result<(), RenderError> {
    let parameter = h.param(0).ok_or(RenderError::new("Parameter is required for format helper"))?;
    let rendered = format!("{} ", parameter.value().render(),);
    out.write(rendered.as_ref())?;
    Ok(())
}

pub fn format_data(tot_util: &f64, srp_results: &Vec<(String, u32, u32, u32, u32)>) -> Map<String, Json> {
    let mut data = Map::new();
    let mut result = Vec::new();
    for i in srp_results {
        let result_holder = ResultHolder {
            id: i.0.to_string(),
            response_time: i.1,
            execution_time: i.2,
            blocking_time: i.3,
            preemption_time: i.4,
        };
        result.push(result_holder);
    }
    data.insert("srp_results".to_string(), to_json(&result));
    data.insert("load".to_string(), to_json(&tot_util.to_string()));
    data
}

pub fn render(load: &f64, srp_results: &Vec<(String, u32, u32, u32, u32)>,) -> Result<(), Box<dyn Error>> {
    let mut handlebars = Handlebars::new();
    handlebars.register_helper("format", Box::new(format));
    let data = format_data(&load, &srp_results);
    handlebars.register_template_file("template", "./src/template.hbs").unwrap();
    let output_file = File::create("target/srp_analysis.html")?;
    handlebars.render_to_write("template", &data, &output_file)?;
    Ok(())
}

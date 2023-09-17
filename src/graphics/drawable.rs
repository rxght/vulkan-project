
use std::sync::Arc;
use vulkano::pipeline::GraphicsPipeline;

#[path ="bindable.rs"]
pub mod bindable;
pub use bindable::Bindable;

pub trait Drawable
{
    fn get_pipeline() -> Arc<GraphicsPipeline>;
    fn get_bindables() -> Vec<Arc<dyn Bindable>>;
}

struct GenericDrawable {

}

impl Drawable for GenericDrawable {

    fn get_pipeline() -> Arc<GraphicsPipeline> {
        todo!()
    }
    
    fn get_bindables() -> Vec<Arc<dyn Bindable>> {
        todo!()
    }
}
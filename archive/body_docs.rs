///
/// # Properties
/// ## Internal
/// * `form`: The polygon's shape; i.e. whether it is a polygon, circle, etc.; certain internal calculations handled differently per form
/// * `position`: The body's global position
/// * `rotation`: The body's rotation from 'true north'. Measured in radians
/// * `origin`: Defines where the body is to be drawn from. SDL2 draws objects from the top-left corner, which is unsuitable for polygons, which use the Cartesian center.
/// * `radius`: The 'size' of polygons & circles. Defines where points are plotted in polygons. Ignore for rect-likes.
///
/// ## Polygons
/// Everything in this section exclusively pertains to bodies with `form: BodyForm::Polygon`.
///
/// * `sides`: How many edges the polygon has. MUST to be 4 for rect-likes.
/// * `vertices`: Internal vector for keeping track of vertex positions.
/// * `width`: The width of a rect-like.
/// * `height`: The height of a rect-like.
///
/// ## Physics
/// Everything in this section contains properties related to physics calculations.
///
/// * `center`: The body's center of mass. Equal to the body's center-point for uniform bodies.
/// * `frozen`: Whether the body's position/rotation is to be ignored at the physics step.
/// * `mass`: The body's mass. Related to the `center` property.
/// * `velocity`
/// * `ang_velocity`
/// * `air_friction`
///

///
/// # Arguments
///
/// * `form`: The body's shape; whether it is a Polygon/Circle/etc.
/// * `position`: The initial global position of the body.
/// * `radius`: Defines the size of polygons/circles. Ignore for rect-likes.
/// * `sides`: How many edges the shape has; used in `BodyForm::Polygon`.
/// * `width`: Defines the width of a rect-like.
/// * `height`: Defines the height of a rect-like.
/// * `mass`: The object's mass.
///
/// # Examples
///
/// ```
/// let body = Body::new(BodyForm::Polygon, v2!(0, 0), 10, 6, None, None, 1);
/// ```
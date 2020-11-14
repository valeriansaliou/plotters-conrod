// plotters-conrod
//
// Conrod backend for Plotters
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: MIT

type ShapeSplitterPoint = [f64; 2];

pub struct ShapeSplitter {
    last_point: ShapeSplitterPoint,
    path_segments: Vec<line_intersection::LineInterval<f64>>,
}

impl ShapeSplitter {
    pub fn try_from(path: &[ShapeSplitterPoint]) -> Result<Self, ()> {
        // Only proceed if we have enough points to form at least a triangle
        if path.len() >= 3 {
            // Map all unique segments for the simplified path
            let mut path_segments: Vec<line_intersection::LineInterval<f64>> = Vec::new();

            for index in 0..(path.len() - 1) {
                let (current_point, next_point) = (path[index], path[index + 1]);

                path_segments.push(line_intersection::LineInterval::line_segment(geo::Line {
                    start: (current_point[0], current_point[1]).into(),
                    end: (next_point[0], next_point[1]).into(),
                }));
            }

            Ok(Self {
                last_point: path[path.len() - 1],
                path_segments,
            })
        } else {
            Err(())
        }
    }

    pub fn collect(&mut self) -> Vec<Vec<ShapeSplitterPoint>> {
        let (mut closed_shapes, mut current_shape_index) = (vec![Vec::new()], 0);

        // Intersect each segment with all of its following segments, iteratively, and \
        //   create paths for closed shapes.
        for index in 0..self.path_segments.len() {
            let ref path_segment = self.path_segments[index];

            // Push opening point
            closed_shapes[current_shape_index]
                .push([path_segment.line.start.x(), path_segment.line.start.y()]);

            for sibling_index in (index + 1)..self.path_segments.len() {
                let ref sibling_path_segment = self.path_segments[sibling_index];

                // The lines are not directly connected? Proceed with intersection check.
                if path_segment.line.end != sibling_path_segment.line.start {
                    let intersection = path_segment
                        .relate(sibling_path_segment)
                        .unique_intersection();

                    // An intersection has been found, the current shape can be closed and yielded
                    if let Some(geo::Point(point_intersect)) = intersection {
                        // Close current closed shape at this point
                        closed_shapes[current_shape_index]
                            .push([point_intersect.x.round(), point_intersect.y.round()]);

                        // Start a new shape at this point (will be closed upon a future iteration)
                        closed_shapes
                            .push(vec![[point_intersect.x.round(), point_intersect.y.round()]]);

                        current_shape_index += 1;
                    }
                }
            }
        }

        // Close the first shape with the last point from the original shape?
        if !closed_shapes.is_empty() && !closed_shapes[0].is_empty() {
            closed_shapes[0].push(self.last_point);
        }

        closed_shapes
    }
}

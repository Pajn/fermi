use pathfinding::astar;
use rusty_machine::linalg::Matrix;
use std::iter::Iterator;
use hlt::types::*;

// pub type DistanceMatrix = Matrix<i32>;

// pub fn calculate_distance(game_map: &GameMap) -> DistanceMatrix {
//   let cell_count = game_map.width * game_map.height;
//   // let cell_count = game_map.width;

//   let distance: Vec<i64> = (0..cell_count)
//     .flat_map(|i| {
//       let iy = i / game_map.width;
//       let ix = i % game_map.width;

//       let start = Location { x: ix, y: iy };
//       let start_production = game_map.get_site(start.clone(), STILL).production as f64;

//       let row: Vec<i64> = (0..cell_count)
//         .map(|j| {
//           let jy = j / game_map.width;
//           let jx = j % game_map.width;

//           let goal = Location { x: jx, y: jy };
//           let goal_strength = game_map.get_site(goal.clone(), STILL).strength as f64;

//           ((goal_strength / start_production) * 1000000.0) as i64
//           // astar(
//           //   &Location { x: ix, y: iy },
//           //   |l| {
//           //     let my_production = game_map.get_site(l.clone(), STILL).production as f64;
//           //     let neighbours: Vec<(Location, i64)> = game_map
//           //       .get_neighbour_sites(l.clone())
//           //       .iter()
//           //       .map(|n| {
//           //         (
//           //           n.location.clone(),
//           //           ((n.strength as f64 / my_production) * 1000000.0) as i64,
//           //         )
//           //       })
//           //       .collect();
//           //     neighbours
//           //   },
//           //   |l| {
//           //     ((goal_strength as f64 / game_map.get_site(l.clone(), STILL).production as f64)
//           //       * 1000000.0) as i64
//           //   },
//           //   |l| *l == goal,
//           // ).unwrap()
//           //   .1
//         })
//         .collect();

//       row
//     })
//     .collect();

//   Matrix::new(cell_count as usize, cell_count as usize, distance);

//   let matrix = Matrix::zeros(cell_count as usize, cell_count as usize);

//   matrix
// }

// fn get_p()

pub fn calculate_value(game_map: &GameMap) -> Matrix<f32> {
  // let cell_count = game_map.width * game_map.height;

  let values: Vec<f32> = (0..game_map.height)
    .flat_map(|y| {
      let row: Vec<f32> = (0..game_map.width)
        .map(|x| {
          let location = Location { x, y };
          let site = game_map.get_site(location, STILL);

          site_production_value(site)
        })
        .collect();

      row
    })
    .collect();

  let mean_value = values.iter().sum() / values.len();

  let values: Vec<f32> = values
    .iter()
    .map(|value| if value > mean_value { *value } else { 0f32 })
    .collect();

  Matrix::new(game_map.height as usize, game_map.width as usize, values)
}

pub fn site_production_value(site: &Site) -> f32 {
  (site.production as f32 * site.production as f32) / site.strength as f32
}

// pub fn site_expansion_value(game_map: &GameMap, edges: Vec<Site>, site: Site) -> f32 {
//   let closest = game_map.contents.iter().flat_map(|row| {
//     row
//       .iter()
//       .filter(|other_site| other_site.owner != game_map.my_id)
//       .filter(|other_site| {
//         let my_distance = distance_matrix[[
//           game_map.get_i(site.location) as usize,
//           game_map.get_i(other_site.location) as usize,
//         ]];
//         let min_distance = edges
//           .iter()
//           .map(|edge| {
//             distance_matrix[[
//               game_map.get_i(edge.location) as usize,
//               game_map.get_i(other_site.location) as usize,
//             ]]
//           })
//           .min()
//           .unwrap();

//         // if my_distance == min_distance {Some((other_site, my_distance))} else {None}
//         my_distance == min_distance
//       })
//   });


//   closest
//     .map(|other_site| {
//       site_production_value(other_site)
//         / distance_matrix[[
//           game_map.get_i(site.location) as usize,
//           game_map.get_i(other_site.location) as usize,
//         ]] as f32
//     })
//     .sum()
// }

// pub fn site_value(
//   game_map: &GameMap,
//   distance_matrix: DistanceMatrix,
//   edges: Vec<Site>,
//   site: Site,
// ) -> f32 {
//   site_production_value(&site) + site_expansion_value(game_map, distance_matrix, edges, site)
// }

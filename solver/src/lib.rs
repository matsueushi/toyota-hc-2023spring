#![allow(unused)]

use proconio::{input, source::Source};
use std::io::BufRead;

use itertools::enumerate;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use std::cmp::Ordering;

const BLOCK: usize = 10000;

#[derive(Clone, Debug)]
pub struct Input {
    w: usize,
    h: usize,
    b: usize,
    d: usize,
    items: Vec<Item>,
}

impl Input {
    pub fn from_source<R: BufRead, S: Source<R>>(mut source: &mut S) -> Self {
        input! {
            from &mut source,
            m:usize,
            w: usize,
            h: usize,
            b: usize,
            d: usize,
            cs: [(usize, usize, usize, usize, String, String); m]
        }

        let mut items = Vec::new();
        for (id, (hi, wi, di, ai, fi, gi)) in enumerate(cs) {
            for _ in 0..ai {
                items.push(Item {
                    id,
                    w: wi,
                    h: hi,
                    d: di,
                    orientation: 0,
                    flippable: fi == "Y",
                    fragile: gi == "N",
                })
            }
        }

        Self { w, h, b, d, items }
    }
}

pub struct Output {
    pub output: String,
    pub score: usize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Vector3D {
    x: usize,
    y: usize,
    z: usize,
}

impl Vector3D {
    fn slide(&self, x: usize, y: usize, z: usize) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
            z: self.z + z,
        }
    }
}

type Space = Vector3D;
type Position = Vector3D;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Rect {
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
}

impl Rect {
    fn intersect(&self, other: &Self) -> bool {
        let x0_max = self.x0.max(other.x0);
        let x1_min = self.x1.min(other.x1);
        let y0_max = self.y0.max(other.y0);
        let y1_min = self.y1.min(other.y1);
        // eprintln!("{} {} {} {}", x0_max, x1_min, y0_max, y1_min);
        x0_max < x1_min && y0_max < y1_min
    }

    fn intersect_area(&self, other: &Self) -> usize {
        let x0_max = self.x0.max(other.x0);
        let x1_min = self.x1.min(other.x1);
        let y0_max = self.y0.max(other.y0);
        let y1_min = self.y1.min(other.y1);
        if x0_max < x1_min && y0_max < y1_min {
            (x1_min - x0_max) * (y1_min - y0_max)
        } else {
            0
        }
    }

    fn slide(&self, x: usize, y: usize) -> Self {
        Self {
            x0: x + self.x0,
            y0: y + self.y0,
            x1: x + self.x1,
            y1: y + self.y1,
        }
    }

    fn isin(&self, x: usize, y: usize) -> bool {
        self.x0 <= x && x <= self.x1 && self.y0 <= y && y <= self.y1
    }
}

/// 荷物
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Item {
    id: usize,
    w: usize,
    h: usize,
    d: usize,
    orientation: usize,
    flippable: bool,
    fragile: bool,
}

impl Item {
    fn new_block(w: usize, h: usize, d: usize) -> Self {
        Self {
            id: BLOCK,
            w,
            h,
            d,
            orientation: 0,
            flippable: false,
            fragile: true,
        }
    }

    fn volume(&self) -> usize {
        self.w * self.h * self.d
    }

    fn rotate(&self) -> Option<Self> {
        if (!self.flippable && self.orientation >= 1) || self.orientation == 5 {
            None
        } else {
            Some(Self {
                id: self.id,
                w: self.w,
                h: self.h,
                d: self.d,
                orientation: self.orientation + 1,
                flippable: self.flippable,
                fragile: self.fragile,
            })
        }
    }

    fn dim_x(&self) -> usize {
        match self.orientation {
            0 => self.w,
            1 => self.h,
            2 => self.d,
            3 => self.h,
            4 => self.d,
            _ => self.w,
        }
    }

    fn dim_y(&self) -> usize {
        match self.orientation {
            0 => self.h,
            1 => self.w,
            2 => self.h,
            3 => self.d,
            4 => self.w,
            _ => self.d,
        }
    }

    fn dim_z(&self) -> usize {
        match self.orientation {
            0 => self.d,
            1 => self.d,
            2 => self.w,
            3 => self.w,
            4 => self.h,
            _ => self.h,
        }
    }

    fn project_x(&self, pos: &Position) -> Rect {
        Rect {
            x0: 0,
            y0: 0,
            x1: self.dim_y(),
            y1: self.dim_z(),
        }
        .slide(pos.y, pos.z)
    }

    fn project_y(&self, pos: &Position) -> Rect {
        Rect {
            x0: 0,
            y0: 0,
            x1: self.dim_x(),
            y1: self.dim_z(),
        }
        .slide(pos.x, pos.z)
    }

    fn project_z(&self, pos: &Position) -> Rect {
        Rect {
            x0: 0,
            y0: 0,
            x1: self.dim_x(),
            y1: self.dim_y(),
        }
        .slide(pos.x, pos.y)
    }

    fn space(&self) -> Space {
        Space {
            x: self.dim_x(),
            y: self.dim_y(),
            z: self.dim_z(),
        }
    }

    fn ground_area(&self) -> usize {
        self.dim_x() * self.dim_y()
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        (!self.fragile, self.volume())
            .cmp(&(!other.fragile, other.volume()))
            .reverse()
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Copy, Clone)]
struct Placement {
    pos: Position,
    item: Item,
}

impl Placement {
    fn is_block(&self) -> bool {
        self.item.id == BLOCK
    }

    fn x_lower(&self) -> usize {
        self.pos.x
    }

    fn x_upper(&self) -> usize {
        self.pos.x + self.item.dim_x()
    }

    fn y_lower(&self) -> usize {
        self.pos.y
    }

    fn y_upper(&self) -> usize {
        self.pos.y + self.item.dim_y()
    }

    fn z_lower(&self) -> usize {
        self.pos.z
    }

    fn z_upper(&self) -> usize {
        self.pos.z + self.item.dim_z()
    }

    fn project_x(&self) -> Rect {
        self.item.project_x(&self.pos)
    }

    fn project_y(&self) -> Rect {
        self.item.project_y(&self.pos)
    }

    fn project_z(&self) -> Rect {
        self.item.project_z(&self.pos)
    }

    /// 長方形の交差判定をする
    fn intersect(&self, other: &Self) -> bool {
        // eprintln!(
        //     "{:?} {:?} {:?} {:?} {:?} {:?}",
        //     self.project_x(),
        //     other.project_x(),
        //     self.project_y(),
        //     other.project_y(),
        //     self.project_z(),
        //     other.project_z()
        // );
        self.project_x().intersect(&other.project_x())
            && self.project_y().intersect(&other.project_y())
            && self.project_z().intersect(&other.project_z())
    }

    /// 上に乗っているか、載せられるか
    /// 地面に接しているときは地面の面積
    fn ground_contact_area(&self, under: &Self) -> Result<usize, ()> {
        // underと接しているか
        if self.z_lower() != under.z_upper() {
            return Ok(0);
        }
        let ground = self.project_z();
        let top = under.project_z();
        if ground.intersect(&top) {
            if under.item.fragile {
                Err(())
            } else {
                Ok(ground.intersect_area(&top))
            }
        } else {
            Ok(0)
        }
    }

    fn vertices(&self) -> Vec<Position> {
        let proj_z = self.project_z();
        let mut vs = Vec::new();
        vs.push(self.pos.slide(self.item.dim_x(), 0, 0));
        vs.push(self.pos.slide(0, self.item.dim_y(), 0));
        if !self.is_block() {
            vs.push(self.pos.slide(0, 0, self.item.dim_z()));
        }
        vs
    }
}

#[derive(Debug)]
struct Packer {
    w: usize,
    h: usize,
    d: usize,
    b: usize,
    blocks: Vec<Placement>,
    packed: Vec<Placement>,
    block_vertices: Vec<Position>,
    vertices: Vec<Position>,
}

impl Packer {
    fn new(w: usize, h: usize, d: usize, b: usize) -> Self {
        // 地面を配置する
        let mut blocks = Vec::new();
        blocks.push(Placement {
            pos: Position { x: 0, y: 0, z: 0 },
            item: Item {
                id: BLOCK,
                w,
                h,
                d: 0, // depth は 0
                orientation: 0,
                flippable: true,
                fragile: false,
            },
        });

        let mut packer = Self {
            w,
            h,
            d,
            b,
            blocks,
            packed: Vec::new(),
            block_vertices: Vec::new(),
            vertices: Vec::new(),
        };
        packer.add_block(0, 0, b);
        packer.add_block(w - b, 0, b);
        packer.add_block(0, h - b, b);
        packer.add_block(w - b, h - b, b);
        packer
    }

    fn add_block(&mut self, x: usize, y: usize, b: usize) {
        let placement = Placement {
            pos: Position { x, y, z: 0 },
            item: Item::new_block(b, b, self.d),
        };
        self.add_vertices(placement.vertices(), true);
        self.blocks.push(placement);
    }

    fn add_vertices(&mut self, vs: Vec<Position>, is_block: bool) {
        // 探索候補の点を加える。
        for v in vs {
            if v.x == self.w || v.y == self.h {
                continue;
            }

            // 他のPlacementへ垂線を下ろす
            let mut z = 0;
            for p in &self.packed {
                if v.z >= p.z_upper() && p.project_z().isin(v.x, v.y) {
                    z = z.max(p.z_upper());
                }
            }

            let nv = Position { x: v.x, y: v.y, z };

            if is_block {
                self.block_vertices.push(nv);
            } else {
                self.vertices.push(nv);
            }
        }
    }

    fn clear(&mut self) {
        self.packed.clear();
        self.vertices.clear();
    }

    fn put_item(&mut self, placement: Placement) {
        // eprintln!("put_item {:?}", placement);
        let mut vs = placement.vertices();
        self.packed.push(placement);
        self.add_vertices(vs, false);
    }

    fn check_allocation(&self, pos: &Position, item: &Item) -> Option<Placement> {
        let placement = Placement {
            pos: *pos,
            item: *item,
        };
        // 境界チェック
        if placement.x_upper() > self.w || placement.y_upper() > self.h {
            return None;
        }

        // 交差するかどうかを調べる
        let mut contact_area = 0; // 接触面積
        for p in self.blocks.iter().chain(self.packed.iter()) {
            if placement.intersect(&p) {
                // eprintln!("intersected {:?}, {:?}", p, placement);
                return None;
            }

            match placement.ground_contact_area(&p) {
                Ok(area) => {
                    // eprintln!("{:?} {:?} {}", p, placement, area);
                    contact_area += area;
                }
                Err(_) => {
                    return None;
                }
            }
        }
        // 面積チェック
        let required = (item.ground_area() as f64 * 0.6).round() as usize;
        if contact_area < required {
            // eprintln!(
            //     "area limitation is not satisfied {} {}",
            //     contact_area, required
            // );
            None
        } else {
            Some(placement)
        }
    }

    fn pack_item(&mut self, vertex: &Position, item: &Item) -> Option<Placement> {
        let mut item = Some(*item);
        loop {
            match item {
                Some(it) => {
                    let ret = self.check_allocation(&vertex, &it);
                    if ret.is_some() {
                        return ret;
                    }
                    item = it.rotate();
                }
                None => return None,
            }
        }
        None
    }

    /// 順番通りに並べる
    fn order_items(packed: &Vec<Placement>) -> Vec<Placement> {
        let mut n_items = packed.len();
        let mut items = Vec::new();
        let mut used = vec![false; n_items];
        while n_items > 0 {
            for i in 0..packed.len() {
                let mut top = true;
                if used[i] {
                    continue;
                }
                for j in 0..packed.len() {
                    if i == j || used[j] {
                        continue;
                    }
                    if packed[j].z_lower() >= packed[i].z_upper()
                        && packed[i].project_z().intersect(&packed[j].project_z())
                    {
                        top = false;
                    }
                }

                if top {
                    used[i] = true;
                    items.push(i);
                    n_items -= 1;
                }
            }
        }

        let mut ordered_items = Vec::new();
        for i in items.iter().rev() {
            ordered_items.push(packed[*i]);
        }
        ordered_items
    }

    fn pack(&mut self, items: Vec<Item>) -> (Vec<Placement>, usize) {
        let since = std::time::Instant::now();
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(42);

        let mut items = items;
        items.sort();

        let mut best_penalty = std::usize::MAX;
        let mut best_packed = Vec::new();

        let mut t = 0.0;
        loop {
            let mut success = true;
            for item in &items {
                let mut vs = self.vertices.clone();
                vs.append(&mut self.block_vertices.clone());
                // もっとも低くなるように積む
                let mut h = std::usize::MAX;
                let mut p = None;
                for v in vs {
                    if let Some(placement) = self.pack_item(&v, &item) {
                        let nh = placement.z_upper();
                        if nh < h {
                            h = nh;
                            p = Some(placement);
                        }
                    }
                }
                if let Some(placement) = p {
                    self.put_item(placement);
                } else {
                    success = false;
                    break;
                }
            }

            if success {
                self.packed = Self::order_items(&self.packed);
                let penalty = evaluate_penalty(&self.packed, self.d);
                if penalty < best_penalty {
                    best_penalty = penalty;
                    std::mem::swap(&mut best_packed, &mut self.packed);
                }
            }

            t = since.elapsed().as_secs_f32();
            if t >= 1.9 {
                break;
            }

            self.clear();
            items.shuffle(&mut rng);
        }

        // eprintln!("success. score = {}", best_penalty);
        (best_packed, best_penalty)
    }
}

fn evaluate_penalty(placements: &Vec<Placement>, d: usize) -> usize {
    let mut score = 1000;
    // max height
    let mut max_h = 0;
    let mut overflow_vol = 0;
    for p in placements {
        let h = p.z_upper();
        max_h = max_h.max(h);
        if h > d {
            overflow_vol += p.item.volume();
        }
    }
    score += max_h;
    for i in 0..placements.len() {
        for j in i + 1..placements.len() {
            if placements[i].item.id > placements[j].item.id {
                score += 1000;
            }
        }
    }
    if max_h > d {
        score += 1_000_000 + 1000 * overflow_vol;
    }
    score
}

pub fn solve(input: &Input) -> Output {
    let mut packer = Packer::new(input.w, input.h, input.d, input.b);
    let (packed, score) = packer.pack(input.items.clone());

    let mut output = String::new();
    for p in packed {
        output.push_str(
            format!(
                "{} {} {} {} {}\n",
                p.item.id, p.item.orientation, p.pos.x, p.pos.y, p.pos.z,
            )
            .as_str(),
        )
    }
    Output { output, score }
}

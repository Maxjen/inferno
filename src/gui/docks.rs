use super::{Widget, Rectangle, EventListener, BorderImage, Text};
use ::resources::{ResourceManager, Texture, Font};
use ::rendering::DrawBatch;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::collections::HashMap;
use glium::glutin::Event;
use glium;

const PADDING: i32 = 5;

#[derive(Debug)]
pub enum NewDockCellPosition {
    NeedNewTable {
        group_id: u32,
        index: usize,
    },

    TableExists {
        table_id: u32,
        index: usize,
    }
}

/*pub struct NewDockCellPosition {
    need_new_table: bool,
    table_id: Option<u32>,
    group_id: Option<u32>,
    index: usize,
}*/

trait DockCell {
    fn get_id(&self) -> u32;
    fn set_parent_id(&mut self, parent_id: u32);
    fn set_position(&mut self, x: i32, y: i32);
    fn get_dimensions(&self) -> (i32, i32);
    fn set_dimensions(&mut self, width: i32, y: i32);
    fn add_to_batch(&self, batch: &mut DrawBatch);
    fn get_dock_at_position(&self, x: i32, y: i32) -> Option<u32>;
    fn get_dock_group_at_position(&self, x: i32, y: i32) -> Option<u32>;
    fn get_new_dock_cell_position(&self, x: i32, y: i32, vertical_align: bool, index: usize) -> Option<NewDockCellPosition>;
    fn print(&self);
}

pub struct DockTable {
    id: u32,
    parent_id: Option<u32>,
    root_widget: Weak<RefCell<Docks>>,
    rect: Rectangle,
    vertical_align: bool,
    dock_cells: Vec<Rc<RefCell<DockCell>>>,
}

impl DockTable {
    fn new(id: u32, root_widget: Weak<RefCell<Docks>>, vertical_align: bool) -> DockTable {
        DockTable {
            id: id,
            parent_id: None,
            root_widget: root_widget,
            rect: Rectangle::new(),
            vertical_align: vertical_align,
            dock_cells: Vec::new(),
        }
    }

    fn add_dock_cell(&mut self, dock_cell: Rc<RefCell<DockCell>>) {
        dock_cell.borrow_mut().set_parent_id(self.id);
        self.dock_cells.push(dock_cell);
    }

    fn insert_dock_cell(&mut self, dock_cell: Rc<RefCell<DockCell>>, index: usize) {
        let mut index = index;
        if index < 0 {
            index = 0;
        } else if index > self.dock_cells.len() {
            index = self.dock_cells.len();
        }
        dock_cell.borrow_mut().set_parent_id(self.id);
        self.dock_cells.insert(index, dock_cell);
    }

    /// Removes the cell with the given id from the table and returns it's index. If there is
    /// no cell with that id None will be returned.
    fn remove_dock_cell(&mut self, id: u32) -> Option<usize> {
        let mut to_remove = None;
        for (i, dc) in self.dock_cells.iter().enumerate() {
            if dc.borrow().get_id() == id {
                to_remove = Some(i);
                break;
            }
        }
        if let Some(to_remove) = to_remove {
            self.dock_cells.remove(to_remove);
        }
        to_remove
    }
}

impl DockCell for DockTable {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn set_parent_id(&mut self, parent_id: u32) {
        self.parent_id = Some(parent_id);
    }

    fn set_position(&mut self, x: i32, y: i32) {
        self.rect.position = (x, y);
        if self.vertical_align {
            let mut height_offset = 0;
            for cell in self.dock_cells.iter() {
                cell.borrow_mut().set_position(x, y - height_offset);
                let (_, cell_height) = cell.borrow().get_dimensions();
                height_offset += cell_height + PADDING;
            }
        } else {
            let mut width_offset = 0;
            for cell in self.dock_cells.iter() {
                cell.borrow_mut().set_position(x + width_offset, y);
                let (cell_width, _) = cell.borrow().get_dimensions();
                width_offset += cell_width + PADDING;
            }
        }
    }

    fn get_dimensions(&self) -> (i32, i32) {
        self.rect.dimensions
    }

    fn set_dimensions(&mut self, width: i32, height: i32) {
        self.rect.dimensions = (width, height);
        let num_cells = self.dock_cells.len() as i32;
        if num_cells > 0 {
            if self.vertical_align {
                let height_per_cell = (height - (num_cells - 1) * PADDING) / num_cells;
                for cell in self.dock_cells.iter() {
                    cell.borrow_mut().set_dimensions(width, height_per_cell);
                }
            } else {
                let width_per_cell = (width - (num_cells - 1) * PADDING) / num_cells;
                for cell in self.dock_cells.iter() {
                    cell.borrow_mut().set_dimensions(width_per_cell, height);
                }
            }
        }
    }

    fn add_to_batch(&self, batch: &mut DrawBatch) {
        for cell in self.dock_cells.iter() {
            cell.borrow().add_to_batch(batch);
        }
    }

    fn get_dock_at_position(&self, x: i32, y: i32) -> Option<u32> {
        if self.rect.contains(x, y) {
            for dc in self.dock_cells.iter() {
                let result = dc.borrow().get_dock_at_position(x, y);
                if result.is_some() {
                    return result;
                }
            }
        }
        None
    }

    fn get_dock_group_at_position(&self, x: i32, y: i32) -> Option<u32> {
        if self.rect.contains(x, y) {
            for dc in self.dock_cells.iter() {
                let result = dc.borrow().get_dock_group_at_position(x, y);
                if result.is_some() {
                    return result;
                }
            }
        }
        None
    }

    fn get_new_dock_cell_position(&self, x: i32, y: i32, vertical_align: bool, index: usize) -> Option<NewDockCellPosition> {
        if self.rect.contains(x, y) {
            println!("({}, {}) in table: {} {} {} {}", x, y, self.rect.position.0, self.rect.position.1,
                                                             self.rect.dimensions.0, self.rect.dimensions.1);
            for (i, dc) in self.dock_cells.iter().enumerate() {
                let result = dc.borrow().get_new_dock_cell_position(x, y, self.vertical_align, i);
                if result.is_some() {
                    return result;
                }
                /*if let Some(mut cell_position) = dc.borrow().get_new_dock_cell_position(x, y, self.vertical_align) {
                    let result;
                    match cell_position {
                        NewDockCellPosition::NeedNewTable{ mut table_id, group_id, index } => {
                            if let None = table_id {
                                table_id = Some(self.id);
                            }
                            result = NewDockCellPosition::NeedNewTable { table_id: table_id, group_id: group_id, index: index };
                        }
                        NewDockCellPosition::TableExists{ mut table_id, mut index } => {
                            if let None = table_id {
                                table_id = Some(self.id);
                                index += i;
                            }
                            result = NewDockCellPosition::TableExists { table_id: table_id, index: index };
                        }
                    }
                    println!("{:?}", result);
                    return Some(result);
                }*/
            }
        }
        None
    }

    fn print(&self) {
        println!("    table: {} {} {} {}", self.rect.position.0, self.rect.position.1,
                                           self.rect.dimensions.0, self.rect.dimensions.1);
        for dc in self.dock_cells.iter() {
            dc.borrow().print();
        }
    }
}

pub struct DockGroup {
    id: u32,
    parent_id: Option<u32>,
    tabs_rect: Rectangle,
    docks_rect: Rectangle,
    docks: Vec<Rc<RefCell<Dock>>>,
    dock_background: BorderImage,
}

impl DockGroup {
    fn new(id: u32, dock_background: Texture) -> DockGroup {
        DockGroup {
            id: id,
            parent_id: None,
            tabs_rect: Rectangle::new(),
            docks_rect: Rectangle::new(),
            docks: Vec::new(),
            dock_background: BorderImage::new(dock_background, 3.0, 3.0, 2.0, 4.0),
        }
    }

    fn add_dock(&mut self, dock: Rc<RefCell<Dock>>) {
        dock.borrow_mut().set_group_id(Some(self.id));
        self.docks.push(dock);
    }

    fn remove_dock(&mut self, id: u32) {
        let mut to_remove = None;
        for (i, d) in self.docks.iter().enumerate() {
            if d.borrow().id == id {
                d.borrow_mut().group_id = None;
                to_remove = Some(i);
                break;
            }
        }
        if let Some(to_remove) = to_remove {
            self.docks.remove(to_remove);
        }
    }

    fn get_tabs_rect(&self) -> Rectangle {
        self.tabs_rect.clone()
    }
}

impl DockCell for DockGroup {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn set_parent_id(&mut self, parent_id: u32) {
        self.parent_id = Some(parent_id);
    }

    fn set_position(&mut self, x: i32, y: i32) {
        self.tabs_rect.position = (x, y);
        self.docks_rect.position = (x, y - 20);
        self.dock_background.set_position((x - 2) as f32, (y + 1 - 20) as f32);
        let mut offset_x = 5;
        for d in self.docks.iter() {
            d.borrow_mut().set_tab_position(x + offset_x, y);
            offset_x += d.borrow().get_tab_width() + 8;
        }
        /*self.dock_tab_selected.set_position((x - 2 + 5) as f32, (y + 1) as f32);
        self.dock_tab_deselected.set_position((x - 2 + 83) as f32, (y + 1) as f32);*/
    }

    fn get_dimensions(&self) -> (i32, i32) {
        (self.tabs_rect.dimensions.0, self.tabs_rect.dimensions.1 + self.docks_rect.dimensions.1)
    }

    fn set_dimensions(&mut self, width: i32, height: i32) {
        self.tabs_rect.dimensions = (width, 20);
        self.docks_rect.dimensions = (width, height - 20);
        self.dock_background.set_size((width + 4) as f32, (height + 4 - 20) as f32);
        /*self.dock_tab_selected.set_size(74.0, 22.0);
        self.dock_tab_deselected.set_size(74.0, 21.0);*/
    }

    fn add_to_batch(&self, batch: &mut DrawBatch) {
        self.dock_background.add_to_batch(batch);
        for d in self.docks.iter() {
            d.borrow().add_to_batch(batch);
        }
        /*self.dock_tab_selected.add_to_batch(batch);
        self.dock_tab_deselected.add_to_batch(batch);*/
    }

    fn get_dock_at_position(&self, x: i32, y: i32) -> Option<u32> {
        if self.tabs_rect.contains(x, y) {
            for d in self.docks.iter() {
                if d.borrow().tab_rect.contains(x, y) {
                    return Some(d.borrow().id);
                }
            }
        }
        None
    }

    fn get_dock_group_at_position(&self, x: i32, y: i32) -> Option<u32> {
        if self.tabs_rect.contains(x, y) {
            return Some(self.id);
        }
        None
    }

    fn get_new_dock_cell_position(&self, x: i32, y: i32, vertical_align: bool, index: usize) -> Option<NewDockCellPosition> {
        if self.docks_rect.contains(x, y) {
            let parent_id = match self.parent_id {
                Some(parent_id) => parent_id,
                None => return None,
            };
            println!("({}, {}) in group: {} {} {} {}", x, y, self.docks_rect.position.0, self.docks_rect.position.1,
                                                             self.docks_rect.dimensions.0, self.docks_rect.dimensions.1);

            let left = x - self.docks_rect.position.0 <= 20;
            let right = self.docks_rect.position.0 + self.docks_rect.dimensions.0 - x <= 20;
            let top = y + self.docks_rect.position.1 <= 20;
            let bottom = -self.docks_rect.position.1 + self.docks_rect.dimensions.1 - y <= 20;

            if vertical_align {
                if top {
                    return Some(NewDockCellPosition::TableExists{
                        table_id: parent_id,
                        index: index,
                    });
                } else if bottom {
                    return Some(NewDockCellPosition::TableExists{
                        table_id: parent_id,
                        index: index + 1,
                    });
                } else if left {
                    return Some(NewDockCellPosition::NeedNewTable{
                        group_id: self.id,
                        index: 0,
                    });
                } else if right {
                    return Some(NewDockCellPosition::NeedNewTable{
                        group_id: self.id,
                        index: 1,
                    });
                }
            } else {
                if top {
                    return Some(NewDockCellPosition::NeedNewTable{
                        group_id: self.id,
                        index: 0,
                    });
                } else if bottom {
                    return Some(NewDockCellPosition::NeedNewTable{
                        group_id: self.id,
                        index: 1,
                    });
                } else if left {
                    return Some(NewDockCellPosition::TableExists{
                        table_id: parent_id,
                        index: index,
                    });
                } else if right {
                    return Some(NewDockCellPosition::TableExists{
                        table_id: parent_id,
                        index: index + 1,
                    });
                }
            }
        }
        None
    }

    fn print(&self) {
        println!("    group: {} {} {} {}", self.docks_rect.position.0, self.docks_rect.position.1,
                                           self.docks_rect.dimensions.0, self.docks_rect.dimensions.1);
    }
}

pub struct Dock {
    id: u32,
    group_id: Option<u32>,
    widget: Option<Rc<RefCell<Widget>>>,
    tab_rect: Rectangle,
    is_selected: bool,
    label: Text,
    dock_tab_selected: BorderImage,
    dock_tab_deselected: BorderImage,
    dont_draw: bool,
}

impl Dock {
    fn new(id: u32, font: Font, label: &str, dock_tab_selected: Texture, dock_tab_deselected: Texture) -> Dock {
        let mut result = Dock {
            id: id,
            group_id: None,
            widget: None,
            tab_rect: Rectangle::new(),
            is_selected: true,
            label: Text::new(font.clone(), label),
            dock_tab_selected: BorderImage::new(dock_tab_selected, 5.0, 5.0, 0.0, 0.0),
            dock_tab_deselected: BorderImage::new(dock_tab_deselected, 5.0, 5.0, 0.0, 0.0),
            dont_draw: false,
        };
        let width = result.label.get_width() as i32 + 16;
        result.set_tab_dimensions(width, 20);
        result
    }

    fn set_group_id(&mut self, group_id: Option<u32>) {
        self.group_id = group_id;
    }

    fn set_tab_position(&mut self, x: i32, y: i32) {
        self.tab_rect.position = (x, y);
        self.label.set_position(x as f32 + 8.0, y as f32 - 15.0);
        self.dock_tab_selected.set_position(x as f32 - 2.0, y as f32 + 1.0);
        self.dock_tab_deselected.set_position(x as f32 - 2.0, y as f32 + 1.0);
    }

    fn set_tab_dimensions(&mut self, width: i32, height: i32) {
        self.tab_rect.dimensions = (width, height);
        self.dock_tab_selected.set_size(width as f32 + 4.0, height as f32 + 2.0);
        self.dock_tab_deselected.set_size(width as f32 + 4.0, height as f32 + 1.0);
    }

    fn set_dont_draw(&mut self, dont_draw: bool) {
        self.dont_draw = dont_draw;
    }

    fn get_tab_width(&self) -> i32 {
        self.tab_rect.dimensions.0
    }

    fn get_visual_clone(&self) -> (Text, BorderImage) {
        (self.label.clone(), self.dock_tab_selected.clone())
    }

    fn add_to_batch(&self, batch: &mut DrawBatch) {
        if !self.dont_draw {
            if self.is_selected {
                self.dock_tab_selected.add_to_batch(batch);
            } else {
                self.dock_tab_deselected.add_to_batch(batch);
            }
            self.label.add_to_batch(batch);
        }
    }
}

struct MoveDock {
    root_widget: Rc<RefCell<Docks>>,
    dock_id: u32,
    mouse_pos: (i32, i32),
    tabs_rect: Option<Rectangle>,
    label: Text,
    border_image: BorderImage,
    border_image_offset: (f32, f32),
}

impl MoveDock {
    fn new(root_widget: Rc<RefCell<Docks>>, dock_id: u32, mouse_x: i32, mouse_y: i32) -> MoveDock {
        let tabs_rect = root_widget.borrow().get_dock_tabs_rect(dock_id);
        let dock = root_widget.borrow().docks.get(&dock_id).unwrap().clone();
        dock.borrow_mut().set_dont_draw(true);
        let (label, border_image) = dock.borrow().get_visual_clone();
        let border_image_position = border_image.get_position();
        MoveDock {
            root_widget: root_widget,
            dock_id: dock_id,
            mouse_pos: (mouse_x, mouse_y),
            tabs_rect: tabs_rect,
            label: label,
            border_image: border_image,
            border_image_offset: (border_image_position.0 - mouse_x as f32, border_image_position.1 + mouse_y as f32),
        }
    }
}

impl EventListener for MoveDock {
    fn handle_event(&mut self, event: Event) -> bool {
        match event {
            Event::MouseMoved(x, y) => {
                let mut remove_tabs_rect = false;
                if let Some(ref tabs_rect) = self.tabs_rect {
                    if !tabs_rect.contains(x, y) {
                        self.root_widget.borrow_mut().remove_dock_from_group(self.dock_id);
                        remove_tabs_rect = true;
                    }
                }
                if remove_tabs_rect {
                    self.tabs_rect = None;
                }
                let maybe_add = self.tabs_rect.is_none();
                if maybe_add {
                    if let Some(group_id) = self.root_widget.borrow().get_dock_group_at_position(x, y) {
                        let group = self.root_widget.borrow().dock_groups.get(&group_id).unwrap().clone();
                        let dock = self.root_widget.borrow().docks.get(&self.dock_id).unwrap().clone();
                        group.borrow_mut().add_dock(dock);
                        self.tabs_rect = Some(group.borrow().get_tabs_rect());
                    }
                }

                if let Some(ref tabs_rect) = self.tabs_rect {
                    self.root_widget.borrow_mut().move_dock_to_position(self.dock_id, x);
                    self.border_image.set_position(x as f32 + self.border_image_offset.0, tabs_rect.position.1 as f32 + 1.0);
                    self.label.set_position(x as f32 + self.border_image_offset.0 + 10.0, tabs_rect.position.1 as f32 - 15.0);
                } else {
                    self.border_image.set_position(x as f32 + self.border_image_offset.0, -y as f32 + self.border_image_offset.1);
                    self.label.set_position(x as f32 + self.border_image_offset.0 + 10.0, -y as f32 + self.border_image_offset.1 - 16.0);
                }
                self.mouse_pos = (x, y);
            }
            Event::MouseInput(glium::glutin::ElementState::Released, glium::glutin::MouseButton::Left) => {
                let dock = self.root_widget.borrow().docks.get(&self.dock_id).unwrap().clone();
                dock.borrow_mut().set_dont_draw(false);
                let cell_position = self.root_widget.borrow().get_new_dock_cell_position(self.mouse_pos.0, self.mouse_pos.1);
                if let Some(cell_position) = cell_position {
                    let new_group_id = self.root_widget.borrow_mut().create_group_from_cell_position(cell_position);
                    let new_group = self.root_widget.borrow().dock_groups.get(&new_group_id).unwrap().clone();
                    new_group.borrow_mut().add_dock(dock);
                }

                return true;
            }
            _ => ()
        }
        false
    }

    fn add_to_batch(&self, batch: &mut DrawBatch) {
        self.border_image.add_to_batch(batch);
        self.label.add_to_batch(batch);
    }
}

struct IndexPool {
    recycled: Vec<u32>,
    next_index: u32,
}

impl IndexPool {
    fn new() -> IndexPool {
        IndexPool {
            recycled: Vec::new(),
            next_index: 0,
        }
    }

    fn get_index(&mut self) -> u32 {
        match self.recycled.pop() {
            Some(index) => index,
            None => {
                self.next_index += 1;
                self.next_index - 1
            }
        }
    }

    fn recycle_index(&mut self, index: u32) {
        if index < self.next_index {
            self.recycled.push(index);
        }
    }
}

pub struct Docks {
    weak_self: Option<Weak<RefCell<Docks>>>,
    rect: Rectangle,
    index_pool: IndexPool,
    dock_tables: HashMap<u32, Rc<RefCell<DockTable>>>,
    dock_groups: HashMap<u32, Rc<RefCell<DockGroup>>>,
    docks: HashMap<u32, Rc<RefCell<Dock>>>,
    dock_table_root: Option<Rc<RefCell<DockTable>>>,

    font: Font,
    dock_background: Texture,
    dock_tab_selected: Texture,
    dock_tab_deselected: Texture,
}

impl Docks {
    pub fn new(resource_manager: &mut ResourceManager) -> Rc<RefCell<Docks>> {
        //let mut index_pool = IndexPool::new();
        //let root_index = index_pool.get_index();

        //let dock_table_root = Rc::new(RefCell::new(DockTable::new(root_index, self.weak_self, false)));

        let result = Rc::new(RefCell::new(Docks {
            weak_self: None,
            rect: Rectangle::new(),
            index_pool: IndexPool::new(),
            dock_tables: HashMap::new(),
            dock_groups: HashMap::new(),
            docks: HashMap::new(),
            dock_table_root: None,

            /*dock_background: BorderImage::new(bg_texture, 3.0, 3.0, 2.0, 4.0),
            dock_tab_selected: BorderImage::new(selected_texture, 5.0, 5.0, 0.0, 0.0),
            dock_tab_deselected: BorderImage::new(deselected_texture, 5.0, 5.0, 0.0, 0.0),*/
            font: resource_manager.create_font("DejaVuSans.ttf", 14).unwrap(),
            dock_background: resource_manager.create_texture("example_images/dock.png").unwrap(),
            dock_tab_selected: resource_manager.create_texture("example_images/dock_tab_selected.png").unwrap(),
            dock_tab_deselected: resource_manager.create_texture("example_images/dock_tab_deselected.png").unwrap(),
        }));
        //result.borrow_mut().dock_tables.insert(root_index, dock_table_root);
        let weak_self = Some(Rc::downgrade(&result));
        result.borrow_mut().weak_self = weak_self;
        let dock_table_root = result.borrow_mut().create_dock_table(false);
        result.borrow_mut().dock_table_root = Some(dock_table_root);
        result
    }

    pub fn create_dock_table(&mut self, vertical_align: bool) -> Rc<RefCell<DockTable>> {
        let weak_self = self.weak_self.as_ref().unwrap();
        let result = Rc::new(RefCell::new(DockTable::new(self.index_pool.get_index(), weak_self.clone(), vertical_align)));
        self.dock_tables.insert(result.borrow().id, result.clone());
        result
    }

    pub fn create_dock_group(&mut self) -> Rc<RefCell<DockGroup>> {
        let result = Rc::new(RefCell::new(
            DockGroup::new(self.index_pool.get_index(), self.dock_background.clone())
        ));
        self.dock_groups.insert(result.borrow().id, result.clone());
        result
    }

    pub fn create_dock(&mut self, label: &str) -> Rc<RefCell<Dock>> {
        let result = Rc::new(RefCell::new(
            Dock::new(self.index_pool.get_index(), self.font.clone(), label, self.dock_tab_selected.clone(), self.dock_tab_deselected.clone())
        ));
        self.docks.insert(result.borrow().id, result.clone());
        result
    }

    pub fn delete_table(&mut self, id: u32) {
        if self.dock_tables.remove(&id).is_some() {
            self.index_pool.recycle_index(id);
        }
    }

    pub fn delete_group(&mut self, id: u32) {
        if self.dock_groups.remove(&id).is_some() {
            self.index_pool.recycle_index(id);
        }
    }

    pub fn delete_dock(&mut self, id: u32) {
        if self.dock_groups.remove(&id).is_some() {
            self.index_pool.recycle_index(id);
        }
    }

    pub fn get_dock_tabs_rect(&self, dock_id: u32) -> Option<Rectangle> {
        let dock = match self.docks.get(&dock_id) {
            Some(dock) => dock,
            None => return None,
        };
        let group_id = match dock.borrow().group_id {
            Some(group_id) => group_id,
            None => return None,
        };
        let group = match self.dock_groups.get(&group_id) {
            Some(group) => group,
            None => return None,
        };
        Some(group.borrow().get_tabs_rect())
    }

    pub fn get_dock_group_at_position(&self, x: i32, y: i32) -> Option<u32> {
        /*if self.rect.contains(x, y) {
            let dock_table_root = match self.dock_table_root {
                Some(ref dock_table_root) => dock_table_root,
                None => return None,
            }
            dock_table_root.borrow().get_dock_group_at_position(x, y)
        } else {
            None
        }*/


        if self.rect.contains(x, y) {
            if let Some(ref dock_table_root) = self.dock_table_root {
                return dock_table_root.borrow().get_dock_group_at_position(x, y);
            }
        }
        None
    }

    pub fn get_new_dock_cell_position(&self, x: i32, y: i32) -> Option<NewDockCellPosition> {
        /*let dock_table_root = match self.dock_table_root {
            Some(ref dock_table_root) => dock_table_root,
            None => return None,
        }*/


        if self.rect.contains(x, y) {
            if let Some(ref dock_table_root) = self.dock_table_root {
                dock_table_root.borrow().print();
                return dock_table_root.borrow().get_new_dock_cell_position(x, y, false, 0);;
            }
        }
        None
    }

    pub fn create_group_from_cell_position(&mut self, cell_position: NewDockCellPosition) -> u32 {
        match cell_position {
            NewDockCellPosition::NeedNewTable { group_id, index } => {
                let new_table_id = self.replace_group_by_table(group_id).unwrap();
                let new_table = self.dock_tables.get(&new_table_id).unwrap().clone();
                let new_group = self.create_dock_group();
                let result = new_group.borrow().id;
                new_table.borrow_mut().insert_dock_cell(new_group, index);
                result
            }
            NewDockCellPosition::TableExists { table_id, index } => {
                let table = self.dock_tables.get(&table_id).unwrap().clone();
                let new_group = self.create_dock_group();
                let result = new_group.borrow().id;
                table.borrow_mut().insert_dock_cell(new_group, index);
                result
            }
        }
    }

    pub fn remove_dock_from_group(&mut self, dock_id: u32) {
        let dock = match self.docks.get(&dock_id) {
            Some(dock) => dock.clone(),
            None => return,
        };
        let group_id = match dock.borrow().group_id {
            Some(group_id) => group_id,
            None => return,
        };
        let group = match self.dock_groups.get(&group_id) {
            Some(group) => group.clone(),
            None => return,
        };
        group.borrow_mut().remove_dock(dock_id);
        let group_len = group.borrow().docks.len();
        if group_len == 0 {
            let table_id = match group.borrow().parent_id {
                Some(table_id) => table_id,
                None => return,
            };
            let mut table = match self.dock_tables.get(&table_id) {
                Some(table) => table.clone(),
                None => return,
            };
            table.borrow_mut().remove_dock_cell(group_id);
            self.delete_group(group_id);
            let mut table_len = table.borrow().dock_cells.len();
            while table_len <= 1 {
                let parent_id = match table.borrow().parent_id {
                    Some(parent_id) => parent_id,
                    None => break,
                };
                let parent_table = match self.dock_tables.get(&parent_id) {
                    Some(parent_table) => parent_table.clone(),
                    None => return,
                };
                if table_len == 0 {
                    parent_table.borrow_mut().remove_dock_cell(table.borrow().id);
                } else if table_len == 1 {
                    let index = match parent_table.borrow_mut().remove_dock_cell(table.borrow().id) {
                        Some(index) => index,
                        None => return,
                    };
                    let child = table.borrow().dock_cells.first().expect("Table has no children!").clone();
                    parent_table.borrow_mut().insert_dock_cell(child, index);
                }
                let to_delete = table.borrow().id;
                table = parent_table;
                table_len = table.borrow().dock_cells.len();
                self.delete_table(to_delete);

                /*let parent_table;
                let parent_id = table.borrow().parent_id;
                if let Some(parent_id) = parent_id {
                    parent_table = self.dock_tables.get(&parent_id).unwrap().clone();
                    if table_len == 0 {
                        parent_table.borrow_mut().remove_dock_cell(table.borrow().id);
                    } else if table_len == 1 {
                        let index = parent_table.borrow_mut().remove_dock_cell(table.borrow().id).unwrap();
                        let child = table.borrow().dock_cells.first().unwrap().clone();
                        parent_table.borrow_mut().insert_dock_cell(child, index);
                    }
                    let to_delete = table.borrow().id;
                    table = parent_table;
                    table_len = table.borrow().dock_cells.len();
                    self.delete_table(to_delete);
                } else {
                    break;
                }*/

            }
        }
    }

    pub fn replace_group_by_table(&mut self, group_id: u32) -> Option<u32> {
        let group = match self.dock_groups.get(&group_id) {
            Some(group) => group.clone(),
            None => return None,
        };
        let parent_id = match group.borrow().parent_id {
            Some(parent_id) => parent_id,
            None => return None,
        };
        let parent_table = match self.dock_tables.get(&parent_id) {
            Some(parent_table) => parent_table.clone(),
            None => return None,
        };
        let vertical_align = !parent_table.borrow().vertical_align;

        let index = match parent_table.borrow_mut().remove_dock_cell(group_id) {
            Some(index) => index,
            None => return None,
        };
        let new_table = self.create_dock_table(vertical_align);
        let result = new_table.borrow().id;
        new_table.borrow_mut().add_dock_cell(group);
        parent_table.borrow_mut().insert_dock_cell(new_table, index);
        Some(result)
    }

    pub fn move_dock_to_position(&mut self, dock_id: u32, x: i32) {
        let dock = match self.docks.get(&dock_id) {
            Some(dock) => dock.clone(),
            None => return,
        };
        let dock_tab_width = dock.borrow().tab_rect.dimensions.0;
        let group_id = match dock.borrow().group_id {
            Some(group_id) => group_id,
            None => return,
        };
        let group = match self.dock_groups.get(&group_id) {
            Some(group) => group,
            None => return,
        };
        /*let group_id = dock.borrow().group_id.unwrap();
        let group = self.dock_groups.get(&group_id).unwrap();*/
        let mut cur_index = 0;
        let mut new_index = None;
        let mut old_index = None;
        let mut right_x = group.borrow().tabs_rect.position.0 + 5;
        for (i, d) in group.borrow().docks.iter().enumerate() {
            if let None = new_index {
                if x < right_x + dock_tab_width {
                    new_index = Some(cur_index);
                }
            }
            if d.borrow().id == dock_id {
                old_index = Some(i);
            } else {
                right_x += d.borrow().tab_rect.dimensions.0 + 8;
                cur_index += 1;
            }
        }
        if let None = new_index {
            new_index = Some(group.borrow().docks.len() - 1);
        }
        /*let new_index = match new_index {
            Some(new_index) => new_index,
            None => return,
        };
        let old_index = match old_index {
            Some(old_index) => old_index,
            None => return,
        };
        if old_index != new_index {
            group.borrow_mut().docks.remove(old_index);
            group.borrow_mut().docks.insert(new_index, dock);
        }*/

        if let Some(new_index) = new_index {
            if let Some(old_index) = old_index {
                if old_index != new_index {
                    group.borrow_mut().docks.remove(old_index);
                    group.borrow_mut().docks.insert(new_index, dock);
                }
            }
        }
    }

    pub fn add_test_docks(&mut self) {
        let dock1 = self.create_dock("Tools");
        let dock2 = self.create_dock("Tools2");
        let dock3 = self.create_dock("View");
        let dock4 = self.create_dock("Properties");
        let dock5 = self.create_dock("Dock5");
        let dock6 = self.create_dock("Dock6");

        let d_table_left = self.create_dock_table(true);
        let d1_left = self.create_dock_group();
        d1_left.borrow_mut().add_dock(dock1);
        let d2_left = self.create_dock_group();
        d2_left.borrow_mut().add_dock(dock2);
        d_table_left.borrow_mut().add_dock_cell(d1_left);
        d_table_left.borrow_mut().add_dock_cell(d2_left);

        let d_middle = self.create_dock_group();
        d_middle.borrow_mut().add_dock(dock3);

        let d_table_right = self.create_dock_table(true);
        let d1_right = self.create_dock_group();
        d1_right.borrow_mut().add_dock(dock4);
        let d2_right = self.create_dock_group();
        d2_right.borrow_mut().add_dock(dock5);
        d2_right.borrow_mut().add_dock(dock6);
        d_table_right.borrow_mut().add_dock_cell(d1_right);
        d_table_right.borrow_mut().add_dock_cell(d2_right);

        if let Some(ref dock_table_root) = self.dock_table_root {
            dock_table_root.borrow_mut().add_dock_cell(d_table_left);
            dock_table_root.borrow_mut().add_dock_cell(d_middle);
            dock_table_root.borrow_mut().add_dock_cell(d_table_right);
        }
    }
}

impl Widget for Docks {
    fn set_position(&mut self, x: i32, y: i32) {
        self.rect.position = (x, y);
        if let Some(ref dock_table_root) = self.dock_table_root {
            dock_table_root.borrow_mut().set_position(x, y);
        }
        //self.button_image.borrow_mut().set_position(x as f32, y as f32);
    }

    fn set_dimensions(&mut self, width: i32, height: i32) {
        self.rect.dimensions = (width, height);
        if let Some(ref dock_table_root) = self.dock_table_root {
            dock_table_root.borrow_mut().set_dimensions(width, height);
        }
        //self.button_image.borrow_mut().set_size(width as f32, height as f32);
    }

    fn add_to_batch(&self, batch: &mut DrawBatch) {
        //self.button_image.borrow().add_to_batch(batch);
        if let Some(ref dock_table_root) = self.dock_table_root {
            dock_table_root.borrow().add_to_batch(batch);
        }
    }

    fn get_highest_priority_child(&self, x: i32, y: i32) -> (i32, Option<Rc<RefCell<Widget>>>) {
        if self.rect.contains(x, y) {
            if let Some(ref weak_self) = self.weak_self {
                return (2, Some(weak_self.upgrade().unwrap()));
            }
        }
        (0, None)
    }

    fn create_event_listener(&self, x: i32, y: i32) -> Option<Box<EventListener>> {
        if let Some(ref dock_table_root) = self.dock_table_root {
            if let Some(dock) = dock_table_root.borrow().get_dock_at_position(x, y) {
                if let Some(ref weak_self) = self.weak_self {
                    return Some(Box::new(MoveDock::new(weak_self.upgrade().unwrap(), dock, x, y)));
                }
            }
        }
        None
    }
}

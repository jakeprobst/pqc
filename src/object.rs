use types::*;

use byteorder::{LittleEndian, WriteBytesExt};
use std::convert::From;

#[derive(Debug)]
pub enum ObjectType {
    SetPlayerLocation,
}

#[derive(Debug)]
pub enum ObjectAttribute {
    Player(u32),
}

#[derive(Debug)]
pub struct Object {
    pub otype: ObjectType,
    pub floor_id: FloorType,
    pub section: u16,
    pub pos: Point,
    pub group: u16,
    pub dir: u32,
    pub obj_id: u32,
    pub action: u32,
    pub attributes: Vec<ObjectAttribute>,
}

impl Object {
    pub fn new(otype: ObjectType, floor_id: FloorType, section: u16, pos: Point) -> Object {
        Object {
            otype: otype,
            floor_id: floor_id,
            section: section,
            pos: pos,
            group: 0,
            dir: 0,
            obj_id: 0,
            action: 0,
            attributes: Vec::new(),
        }
    }

    pub fn pos(mut self, pos: Point) -> Object {
        self.pos = pos;
        self
    }

    pub fn group(mut self, group: u16) -> Object {
        self.group = group;
        self
    }
    
    pub fn dir(mut self, dir: u32) -> Object {
        self.dir = dir;
        self
    }

    pub fn obj_id(mut self, obj_id: u32) -> Object {
        self.obj_id = obj_id;
        self
    }
    
    pub fn action(mut self, action: u32) -> Object {
        self.action = action;
        self
    }

    pub fn attribute(mut self, attribute: ObjectAttribute) -> Object {
        self.attributes.push(attribute);
        self
    }

    /*pub fn as_object(&self) -> Object {
        Object{
            otype: self.otype,
            floor_id: self.floor_id,
            section: self.section,
            pos: self.pos,
            group: self.group,
            dir: self.dir,
            obj_id: self.obj_id,
            action: self.action,
            attributes: self.attributes,
        }
    }*/

}


/*pub struct ObjectBuilder {
    otype: ObjectType,
    floor_id: FloorType,
    section: u16,
    pos: Point,
    group: u16,
    dir: u32,
    obj_id: u32,
    action: u32,
    attributes: Vec<ObjectAttribute>,
}

impl ObjectBuilder {
    pub fn new(otype: ObjectType, floor_id: FloorType, section: u16, pos: Point) -> ObjectBuilder {
        ObjectBuilder {
            otype: otype,
            floor_id: floor_id,
            section: section,
            pos: pos,
            group: 0,
            dir: 0,
            obj_id: 0,
            action: 0,
            attributes: Vec::new(),
        }
    }

    pub fn pos<'a>(&'a mut self, pos: Point) -> &'a mut ObjectBuilder {
        self.pos = pos;
        self
    }

    pub fn group<'a>(&'a mut self, group: u16) -> &'a mut ObjectBuilder {
        self.group = group;
        self
    }
    
    pub fn dir<'a>(&'a mut self, dir: u32) -> &'a mut ObjectBuilder {
        self.dir = dir;
        self
    }

    pub fn obj_id<'a>(&'a mut self, obj_id: u32) -> &'a mut ObjectBuilder {
        self.obj_id = obj_id;
        self
    }
    
    pub fn action<'a>(&'a mut self, action: u32) -> &'a mut ObjectBuilder {
        self.action = action;
        self
    }

    pub fn attribute<'a>(&'a mut self, attribute: ObjectAttribute) -> &'a mut ObjectBuilder {
        self.attributes.push(attribute);
        self
    }

    pub fn as_object(&self) -> Object {
        Object{
            otype: self.otype,
            floor_id: self.floor_id,
            section: self.section,
            pos: self.pos,
            group: self.group,
            dir: self.dir,
            obj_id: self.obj_id,
            action: self.action,
            attributes: self.attributes,
        }
    }

}
*/

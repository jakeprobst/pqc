use types::*;

use byteorder::{LittleEndian, WriteBytesExt};
use std::convert::From;

#[derive(Debug, Clone)]
pub enum ObjectType {
    SetPlayerLocation,
}

impl From<ObjectType> for u16 {
    fn from(objtype: ObjectType) -> u16 {
        match objtype {
            ObjectType::SetPlayerLocation => 0,
        }
    }
}



#[derive(Debug)]
pub enum ObjectAttribute {
    Player(u32),
}

#[derive(Debug)]
pub struct Object {
    pub otype: ObjectType,
    pub id: u16,
    pub floor: FloorType,
    pub section: u16,
    pub pos: Point,
    pub group: u16,
    pub dir: u32,
    pub obj_id: u32,
    pub action: u32,
    pub attributes: Vec<ObjectAttribute>,
}

impl Object {
    pub fn new(otype: ObjectType, id: u16, floor: FloorType, section: u16, pos: Point) -> Object {
        Object {
            otype: otype,
            id: id,
            floor: floor,
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
}

impl<'a> From<&'a Object> for Vec<u8> {
    fn from(obj: &'a Object) -> Vec<u8> {
        match obj.otype {
            ObjectType::SetPlayerLocation => raw_set_player_location(&obj),
            
        }
    }
}

struct RawObjectData {
    otype: u16,
    unknown1: u16,
    unknown2:  u32,
    id: u16,
    group: u16,
    section: u16,
    unknown3: u16,
    x: f32,
    y: f32,
    z: f32,
    xrot: u32,
    yrot: u32,
    zrot: u32,
    field1: u32,
    field2: u32,
    field3: u32,
    field4: f32,
    field5: f32,
    field6: f32,
    obj_id: u32,
    action: u32,
    field7: u32,
    field8: u32,
}

// TODO: what to do with id?
impl RawObjectData {
    pub fn new(obj: &Object) -> RawObjectData {
        RawObjectData {
            otype: u16::from(obj.otype.clone()),
            unknown1: 0,
            unknown2: 0,
            id: obj.id,
            group: 0,
            section: obj.section,
            unknown3: 0,
            x: obj.pos.x,
            y: obj.pos.y,
            z: obj.pos.z,
            xrot: 0,
            yrot: obj.dir,
            zrot: 0,
            field1: 0,
            field2: 0,
            field3: 0,
            field4: 0.0,
            field5: 0.0,
            field6: 0.0,
            obj_id: 0,
            action: 0,
            field7: 0,
            field8: 0,
            
        }
    }

    pub fn unknown1<'a>(&'a mut self, unknown1: u16) -> &'a mut RawObjectData {
        self.unknown1 = unknown1;
        self
    }
    
    pub fn unknown2<'a>(&'a mut self, unknown2: u32) -> &'a mut RawObjectData {
        self.unknown2 = unknown2;
        self
    }
    
    pub fn group<'a>(&'a mut self, group: u16) -> &'a mut RawObjectData {
        self.group = group;
        self
    }

    pub fn section<'a>(&'a mut self, section: u16) -> &'a mut RawObjectData {
        self.section = section;
        self
    }

    pub fn unknown3<'a>(&'a mut self, unknown3: u16) -> &'a mut RawObjectData {
        self.unknown3 = unknown3;
        self
    }

    pub fn xrot<'a>(&'a mut self, xrot: u32) -> &'a mut RawObjectData {
        self.xrot = xrot;
        self
    }

    pub fn zrot<'a>(&'a mut self, zrot: u32) -> &'a mut RawObjectData {
        self.zrot = zrot;
        self
    }

    pub fn field1<'a>(&'a mut self, field1: u32) -> &'a mut RawObjectData {
        self.field1 = field1;
        self
    }

    pub fn field2<'a>(&'a mut self, field2: u32) -> &'a mut RawObjectData {
        self.field2 = field2;
        self
    }

    pub fn field3<'a>(&'a mut self, field3: u32) -> &'a mut RawObjectData {
        self.field3 = field3;
        self
    }

    pub fn field4<'a>(&'a mut self, field4: f32) -> &'a mut RawObjectData {
        self.field4 = field4;
        self
    }

    pub fn field5<'a>(&'a mut self, field5: f32) -> &'a mut RawObjectData {
        self.field5 = field5;
        self
    }

    pub fn field6<'a>(&'a mut self, field6: f32) -> &'a mut RawObjectData {
        self.field6 = field6;
        self
    }

    pub fn obj_id<'a>(&'a mut self, obj_id: u32) -> &'a mut RawObjectData {
        self.obj_id = obj_id;
        self
    }

    pub fn action<'a>(&'a mut self, action: u32) -> &'a mut RawObjectData {
        self.action = action;
        self
    }

    pub fn field7<'a>(&'a mut self, field7: u32) -> &'a mut RawObjectData {
        self.field7 = field7;
        self
    }

    pub fn field8<'a>(&'a mut self, field8: u32) -> &'a mut RawObjectData {
        self.field8 = field8;
        self
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut odata = Vec::new();
        odata.write_u16::<LittleEndian>(self.otype);
        odata.write_u16::<LittleEndian>(self.unknown1);
        odata.write_u32::<LittleEndian>(self.unknown2);
        odata.write_u16::<LittleEndian>(self.id);
        odata.write_u16::<LittleEndian>(self.group);
        odata.write_u16::<LittleEndian>(self.section);
        odata.write_u16::<LittleEndian>(self.unknown3);
        odata.write_f32::<LittleEndian>(self.x);
        odata.write_f32::<LittleEndian>(self.y);
        odata.write_f32::<LittleEndian>(self.z);
        odata.write_u32::<LittleEndian>(self.xrot);
        odata.write_u32::<LittleEndian>(self.yrot);
        odata.write_u32::<LittleEndian>(self.zrot);
        odata.write_u32::<LittleEndian>(self.field1);
        odata.write_u32::<LittleEndian>(self.field2);
        odata.write_u32::<LittleEndian>(self.field3);
        odata.write_f32::<LittleEndian>(self.field4);
        odata.write_f32::<LittleEndian>(self.field5);
        odata.write_f32::<LittleEndian>(self.field6);
        odata.write_u32::<LittleEndian>(self.obj_id);
        odata.write_u32::<LittleEndian>(self.action);
        odata.write_u32::<LittleEndian>(self.field7);
        odata.write_u32::<LittleEndian>(self.field8);
        odata
    }
}

// TODO: return flag
fn raw_set_player_location(obj: &Object) -> Vec<u8> {
    let mut slot_id = 0;
    for attr in obj.attributes.iter() {
        match attr {
            &ObjectAttribute::Player(player) => slot_id = player,
            //_ => {}
        }
    }
    RawObjectData::new(&obj)
        .field1(slot_id)
        .as_bytes()
}

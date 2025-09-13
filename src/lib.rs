use mlua::prelude::*;
use thimni::{
    sdf::{CollisionResult, SDF},
    utils::{AABB, CollisionParameters},
};

pub trait LuaTableWrapper {
    fn to_vec2(&self) -> [f32; 2];
    fn to_vec3(&self) -> [f32; 3];
    fn to_params(&self) -> CollisionParameters;
}

impl LuaTableWrapper for LuaTable {
    fn to_vec2(&self) -> [f32; 2] {
        [self.get::<f32>("x").unwrap(), self.get::<f32>("y").unwrap()]
    }

    fn to_vec3(&self) -> [f32; 3] {
        [
            self.get::<f32>("x").unwrap(),
            self.get::<f32>("y").unwrap(),
            self.get::<f32>("z").unwrap(),
        ]
    }

    fn to_params(&self) -> CollisionParameters {
        let mut params = CollisionParameters::default();

        if let Some(a) = self.get::<f32>("normal_epsilon").ok() {
            params.normal_epsilon = a;
        }
        if let Some(a) = self.get::<f32>("learning_rate").ok() {
            params.learning_rate = a;
        }
        if let Some(a) = self.get::<f32>("collision_epsilon").ok() {
            params.collision_epsilon = a;
        }
        if let Some(a) = self.get::<usize>("max_gds_iter").ok() {
            params.max_gds_iter = a;
        }
        if let Some(a) = self.get::<usize>("max_packing_iter").ok() {
            params.max_packing_iter = a;
        }
        if let Some(a) = self.get::<f32>("area_percentage").ok() {
            params.area_percentage = a;
        }
        if let Some(a) = self.get::<f32>("minimum_radius").ok() {
            params.minimum_radius = a;
        }

        params
    }
}

// A wrapper around a 3D SDF, represented by a table.
// A 3D SDF table must have the following:
// A method called dist, that takes a 3D position and returns a number
// A method called get_aabb, that returns the AABB of the SDF.
// The AABB is represented as a table with a min and a max element.
#[derive(Clone)]
pub struct LuaSDF3D<'lua> {
    pub table: LuaTable,
    pub lua: &'lua Lua,
}

impl<'lua> LuaSDF3D<'lua> {
    pub fn vec_into_table(&self, vec: &[f32; 3]) -> LuaTable {
        let v = self.lua.create_table().unwrap();

        v.set("x", vec[0]).unwrap();
        v.set("y", vec[1]).unwrap();
        v.set("z", vec[2]).unwrap();

        v
    }
}

impl<'lua> SDF<3, [f32; 3]> for LuaSDF3D<'lua> {
    fn dist(&self, p: [f32; 3]) -> f32 {
        let pt = self.vec_into_table(&p);

        self.table.call_method("dist", pt).unwrap()
    }

    fn aabb(&self) -> thimni::utils::AABB<3, [f32; 3]> {
        let aabb: LuaTable = self.table.call_method("get_aabb", ()).unwrap();

        let min: [f32; 3] = aabb.get::<LuaTable>("min").unwrap().to_vec3();
        let max: [f32; 3] = aabb.get::<LuaTable>("max").unwrap().to_vec3();

        AABB { min, max }
    }
}

// A wrapper around a 2D SDF, represented by a table.
// A 2D SDF table must have the following:
// A method called dist, that takes a 2D position and returns a number
// A method called get_aabb, that returns the AABB of the SDF.
// The AABB is represented as a table with a min and a max element.
#[derive(Clone)]
pub struct LuaSDF2D<'lua> {
    pub table: LuaTable,
    pub lua: &'lua Lua,
}

impl<'lua> LuaSDF2D<'lua> {
    pub fn vec_into_table(&self, vec: &[f32; 2]) -> LuaTable {
        let v = self.lua.create_table().unwrap();

        v.set("x", vec[0]).unwrap();
        v.set("y", vec[1]).unwrap();

        v
    }
}

impl<'lua> SDF<2, [f32; 2]> for LuaSDF2D<'lua> {
    fn dist(&self, p: [f32; 2]) -> f32 {
        let pt = self.vec_into_table(&p);

        self.table.call_method("dist", pt).unwrap()
    }

    fn aabb(&self) -> thimni::utils::AABB<2, [f32; 2]> {
        let aabb: LuaTable = self.table.call_method("get_aabb", ()).unwrap();

        let min: [f32; 2] = aabb.get::<LuaTable>("min").unwrap().to_vec2();
        let max: [f32; 2] = aabb.get::<LuaTable>("max").unwrap().to_vec2();

        AABB { min, max }
    }
}

fn get_collision_3d(
    lua: &Lua,
    (params, a, b): (LuaTable, LuaTable, LuaTable),
) -> LuaResult<LuaTable> {
    let asdf = LuaSDF3D {
        table: a,
        lua: &lua,
    };

    let bsdf = LuaSDF3D {
        table: b,
        lua: &lua,
    };

    let params = params.to_params();

    let result = asdf.get_coll_point(&bsdf, &params);

    let rt = lua.create_table().unwrap();

    if let Some(res) = result {
        let point = lua.create_table().unwrap();
        point.set("x", res.point[0])?;
        point.set("y", res.point[1])?;
        point.set("z", res.point[2])?;

        let grad = lua.create_table().unwrap();
        grad.set("x", res.gradient[0])?;
        grad.set("y", res.gradient[1])?;
        grad.set("z", res.gradient[2])?;

        rt.set("point", point)?;
        rt.set("gradient", grad)?;
    }

    Ok(rt)
}

fn approximate_depth_3d(
    lua: &Lua,
    (params, a, b, result): (LuaTable, LuaTable, LuaTable, LuaTable),
) -> LuaResult<f32> {
    let asdf = LuaSDF3D {
        table: a,
        lua: &lua,
    };

    let bsdf = LuaSDF3D {
        table: b,
        lua: &lua,
    };

    let params = params.to_params();

    let depth = asdf.sum_gradient_depth(
        &bsdf,
        &params,
        &CollisionResult {
            point: result.get::<LuaTable>("point").unwrap().to_vec3(),
            gradient: result.get::<LuaTable>("gradient").unwrap().to_vec3(),
        },
    );

    Ok(depth)
}

fn get_collision_2d(
    lua: &Lua,
    (params, a, b): (LuaTable, LuaTable, LuaTable),
) -> LuaResult<LuaTable> {
    let asdf = LuaSDF2D {
        table: a,
        lua: &lua,
    };

    let bsdf = LuaSDF2D {
        table: b,
        lua: &lua,
    };

    let params = params.to_params();

    let result = asdf.get_coll_point(&bsdf, &params);

    let rt = lua.create_table().unwrap();

    if let Some(res) = result {
        let point = lua.create_table().unwrap();
        point.set("x", res.point[0])?;
        point.set("y", res.point[1])?;

        let grad = lua.create_table().unwrap();
        grad.set("x", res.gradient[0])?;
        grad.set("y", res.gradient[1])?;

        rt.set("point", point)?;
        rt.set("gradient", grad)?;
    }

    Ok(rt)
}

fn approximate_depth_2d(
    lua: &Lua,
    (params, a, b, result): (LuaTable, LuaTable, LuaTable, LuaTable),
) -> LuaResult<f32> {
    let asdf = LuaSDF2D {
        table: a,
        lua: &lua,
    };

    let bsdf = LuaSDF2D {
        table: b,
        lua: &lua,
    };

    let params = params.to_params();

    let depth = asdf.sum_gradient_depth(
        &bsdf,
        &params,
        &CollisionResult {
            point: result.get::<LuaTable>("point").unwrap().to_vec2(),
            gradient: result.get::<LuaTable>("gradient").unwrap().to_vec2(),
        },
    );

    Ok(depth)
}

#[mlua::lua_module]
fn limni(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    exports.set("get_collision_3d", lua.create_function(get_collision_3d)?)?;
    exports.set("get_collision_2d", lua.create_function(get_collision_2d)?)?;

    exports.set(
        "approximate_depth_3d",
        lua.create_function(approximate_depth_3d)?,
    )?;

    exports.set(
        "approximate_depth_2d",
        lua.create_function(approximate_depth_2d)?,
    )?;

    Ok(exports)
}

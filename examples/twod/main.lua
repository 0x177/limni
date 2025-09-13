limni = require("limni")

function vector_length(x, y)
    return math.sqrt(x ^ 2 + y ^ 2)
end

function create_circle(px,py,pr)
    j =  {
        pos = {x = px, y = py},
        radius = pr
    }

    j.dist = function (self, ps)
        local dx = ps.x - self.pos.x
        local dy = ps.y - self.pos.y
        
        return vector_length(dx, dy) - self.radius
    end

    j.get_aabb = function (self)
        local amin = {x = self.pos.x - self.radius, y = self.pos.y - self.radius}
        local amax = {x = self.pos.x + self.radius, y = self.pos.y + self.radius}

        return {min = amin, max = amax}
    end

    return j
end

circle1 = create_circle(200,300,50)
circle2 = create_circle(500,300,30)

circle2["velocity"] = {x = 0.0, y = 0.0}
circle2["speed"] = 35.0


function create_rectangle(px,py,pw,ph)
    a =  {
        pos = {x = px, y = py},
        size = {w = pw, h = ph}
    }

    a.dist = function (self, p)
        bx = self.size.w * 0.5
        by = self.size.h * 0.5

        cx = self.pos.x + bx
        cy = self.pos.y + by

        wx = math.abs(p.x - cx) - bx
        wy = math.abs(p.y - cy) - by

        return (vector_length(math.max(wx,0.0),math.max(wy,0.0)) + math.min(math.max(wx, wy),0.0))
    end

    a.get_aabb = function (self)
        bottom_right_x = self.pos.x + self.size.w
        bottom_right_y = self.pos.y + self.size.h

        return {min = self.pos, max = {x = bottom_right_x, y = bottom_right_y}}
    end

    return a
end

rectangle1 = create_rectangle(250,400,100,100)

-- empty table = default parameters
collision_params = {}

function love.load()
    love.window.setTitle("thimni + lua + LÖVE")
end

function love.update(dt)
    
    if love.keyboard.isDown("w") then
        circle2.velocity.y = circle2.velocity.y - 1.0
    end
    if love.keyboard.isDown("s") then
        circle2.velocity.y = circle2.velocity.y + 1.0
    end
    if love.keyboard.isDown("a") then
        circle2.velocity.x = circle2.velocity.x - 1.0
    end
    if love.keyboard.isDown("d") then
        circle2.velocity.x = circle2.velocity.x + 1.0
    end

    circle2.pos.x = circle2.pos.x + circle2.velocity.x * dt * circle2.speed
    circle2.pos.y = circle2.pos.y + circle2.velocity.y * dt * circle2.speed

    circle2.velocity.x = 0.0
    circle2.velocity.y = 0.0

    result = limni.get_collision_2d(collision_params,circle2,circle1)

    if result.point ~= nil then
        depth = limni.approximate_depth_2d(collision_params,circle2,circle1,result)
        
        circle2.velocity.x = result.gradient.x * depth
        circle2.velocity.y = result.gradient.y * depth
    end
    
    result2 = limni.get_collision_2d(collision_params,circle2,rectangle1)

    if result2.point ~= nil then
        depth = limni.approximate_depth_2d(collision_params,circle2,rectangle1,result2)
        
        circle2.velocity.x = result2.gradient.x * depth
        circle2.velocity.y = result2.gradient.y * depth
    end
end

function love.draw()
    love.graphics.clear(0, 0, 0)

    love.graphics.setColor(1, 1, 0)

    love.graphics.circle("fill", circle1.pos.x, circle1.pos.y, circle1.radius)

    love.graphics.setColor(0, 1, 1)
    love.graphics.circle("fill", circle2.pos.x, circle2.pos.y, circle2.radius)

    love.graphics.setColor(1, 0, 1)
    
    love.graphics.rectangle("fill", rectangle1.pos.x, rectangle1.pos.y, rectangle1.size.w, rectangle1.size.h)
end

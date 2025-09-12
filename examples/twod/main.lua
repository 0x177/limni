limni = require("limni")

function create_circle(px,py,pr)
    j =  {
        pos = {x = px, y = py},
        radius = pr
    }

    j.dist = function (self, ps)
        local dx = ps.x - self.pos.x
        local dy = ps.y - self.pos.y
        
        return math.sqrt(dx ^ 2 + dy ^ 2) - self.radius
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
    return {
        pos = {x = px, y = py},
        size = {w = pw, h = ph}
    }
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
        circle2.velocity.x = result.gradient.x
        circle2.velocity.y = result.gradient.y
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

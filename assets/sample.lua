ON = 1
OFF = 0

function main()
boosted = false

if api_is_pressed("w") then
    api_boost("WL", ON)
    api_boost("WR", ON)
    boosted = true
end

if api_is_pressed("s") then
    api_boost("FL", ON)
    api_boost("FR", ON)
    boosted = true
end

if api_is_pressed("a") then
    api_boost("FL", ON)
    api_boost("BR", ON)
    boosted = true
end

if api_is_pressed("d") then
    api_boost("BL", ON)
    api_boost("FR", ON)
    boosted = true
end

if not boosted then
    api_boost("BL", OFF)
    api_boost("BR", OFF)
    api_boost("FL", OFF)
    api_boost("FR", OFF)
    api_boost("WL", OFF)
    api_boost("WR", OFF)
end

return ""
end

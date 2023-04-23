ON = 1
OFF = 0

function main()
    boosted = false

    if api.is_pressed("w") then
        api.boost("WL", ON)
        api.boost("WR", ON)
        boosted = true
    end

    if api.is_pressed("s") then
        api.boost("FL", ON)
        api.boost("FR", ON)
        boosted = true
    end

    if api.is_pressed("a") then
        api.boost("FL", ON)
        api.boost("BR", ON)
        boosted = true
    end

    if api.is_pressed("d") then
        api.boost("BL", ON)
        api.boost("FR", ON)
        boosted = true
    end

    if not boosted then
        api.boost("BL", OFF)
        api.boost("BR", OFF)
        api.boost("FL", OFF)
        api.boost("FR", OFF)
        api.boost("WL", OFF)
        api.boost("WR", OFF)
    end

    return ""
end

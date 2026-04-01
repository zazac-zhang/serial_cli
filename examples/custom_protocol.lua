-- Example Lua protocol definition
-- This shows how to define a custom protocol in Lua

-- Simple frame protocol with header and checksum
local function on_frame(data)
    -- Parse incoming frame
    if #data < 3 then
        return nil  -- Frame too short
    end

    -- Verify header
    if data[1] ~= 0xAA then
        return nil  -- Invalid header
    end

    -- Verify checksum
    local checksum = 0
    for i = 1, #data - 1 do
        checksum = checksum + data[i]
    end
    checksum = checksum % 256

    if checksum ~= data[#data] then
        return nil  -- Checksum mismatch
    end

    -- Return payload (without header and checksum)
    local payload = {}
    for i = 2, #data - 1 do
        table.insert(payload, data[i])
    end
    return payload
end

local function on_encode(data)
    -- Encode outgoing frame with header and checksum
    local frame = {0xAA}  -- Header
    local checksum = 0xAA

    -- Add payload
    for i = 1, #data do
        table.insert(frame, data[i])
        checksum = checksum + data[i]
    end

    -- Add checksum
    checksum = checksum % 256
    table.insert(frame, checksum)

    return frame
end

-- Export functions
return {
    on_frame = on_frame,
    on_encode = on_encode
}

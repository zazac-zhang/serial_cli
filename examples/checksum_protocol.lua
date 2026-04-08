-- Example: Custom protocol with checksum
-- This demonstrates a simple protocol with frame validation

-- Protocol: checksum_protocol
local checksum = require("checksum")

function on_frame(data)
    -- Validate checksum
    if #data < 3 then
        return nil  -- Invalid frame
    end

    local received_checksum = data:byte(#data)
    local calculated_checksum = 0

    for i = 1, #data - 1 do
        calculated_checksum = (calculated_checksum + data:byte(i)) % 256
    end

    if received_checksum ~= calculated_checksum then
        log_warn(string.format("Checksum mismatch: got 0x%02X, expected 0x%02X",
                               received_checksum, calculated_checksum))
        return data  -- Return data anyway for error handling
    end

    -- Return payload without checksum
    return data:sub(1, #data - 1)
end

function on_encode(data)
    -- Calculate and append checksum
    local checksum = 0
    for i = 1, #data do
        checksum = (checksum + data:byte(i)) % 256
    end

    return data .. string.char(checksum)
end

function on_reset()
    log_info("Checksum protocol reset")
end

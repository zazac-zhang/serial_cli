-- Protocol: test_valid
-- Valid test protocol with all required functions

function on_frame(data)
    -- Simply return the data as-is
    return data
end

function on_encode(data)
    -- Simply return the data as-is
    return data
end

function on_reset()
    -- Optional reset callback
end

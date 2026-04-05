-- Protocol: test_missing_func
-- Protocol missing required on_encode function

function on_frame(data)
    return data
end

-- Missing on_encode function

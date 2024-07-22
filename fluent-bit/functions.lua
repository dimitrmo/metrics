function bool_to_number(tag, timestamp, record)
    if record["alive"] ~= nil then
        record["alive_as_number"] = record["alive"] and 1 or 0
    end

    return 1, timestamp, record
end

function go_bool_to_number(tag, timestamp, record)
    if record["alive"] ~= nil then
        record["alive_as_number"] = record["alive"] and 1 or 0
    end

    record["instance"] = "ws-server-go:8080"
    record["job"] = "health.ws-server-go"

    return 1, timestamp, record
end

function rs_bool_to_number(tag, timestamp, record)
    if record["alive"] ~= nil then
        record["alive_as_number"] = record["alive"] and 1 or 0
    end

    record["instance"] = "ws-server-rs:8081"
    record["job"] = "health.ws-server-rs"

    return 1, timestamp, record
end

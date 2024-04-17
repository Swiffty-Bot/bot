_G Swiffty = {}

Swiffty.commands = {}
Swiffty.events = {}

function Swiffty:registerCommand(name, callback)
    self.commands[name] = callback
end

function Swiffty:registerEvent(name)
    self.events[name] = {}
end

function Swiffty:registerEventHandler(eventName, callback)
    table.insert(self.events[eventName], callback)
end

function Swiffty:triggerEvent(eventName, ...)
    for _, callback in ipairs(self.events[eventName]) do
        callback(...)
    end
end

function Swiffty:runCommand(name, ...)
    self.commands[name](...)
end
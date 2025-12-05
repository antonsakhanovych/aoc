const std = @import("std");

const Direction = enum {
    left,
    right,
    fn get_direction(letter: u8) Direction {
        return switch (letter) {
            'L' => .left,
            'R' => .right,
            else => unreachable,
        };
    }
};

const Command = struct {
    direction: Direction,
    circles: u8,
    clicks: u8,
    fn parse_line(line: []const u8) !Command {
        const dir = Direction.get_direction(line[0]);
        const clicks = try std.fmt.parseInt(u32, line[1..], 10);
        return .{
            .direction = dir,
            .circles = @intCast(@divTrunc(clicks, 100)),
            .clicks = @intCast(clicks % 100),
        };
    }
};

const Lock = struct {
    state: i16,

    fn execute_command(self: *Lock, command: Command) u32 {
        var zero_passes: u32 = command.circles;
        const prev_state = self.state;
        switch (command.direction) {
            .left => {
                self.state -= command.clicks;
            },
            .right => {
                self.state += command.clicks;
            },
        }

        if (self.state > 99) {
            self.state -= 100;
            if (self.state != 0) {
                zero_passes += 1;
            }
        } else if (self.state < 0) {
            self.state += 100;
            if (prev_state != 0) {
                zero_passes += 1;
            }
        }
        if (zero_passes > 1 and prev_state == 0 and self.state == 0) {
            zero_passes -= 1;
        }
        return zero_passes;
    }

    fn is_zero(self: *const Lock) bool {
        return self.state == 0;
    }
};

pub fn main() !void {
    const argv = std.os.argv;
    var stderr = std.fs.File.stderr().writer(&.{});
    var stdout = std.fs.File.stdout().writer(&.{});
    if (argv.len < 2) {
        try stderr.interface.print("Please provide a file!", .{});
        std.process.exit(1);
    }
    const file_path = std.mem.span(argv[1]);

    const cwd = std.fs.cwd();

    const file = try cwd.openFile(file_path, .{ .mode = .read_only });
    defer file.close();
    var file_buffer: [4096]u8 = undefined;
    var reader = file.reader(&file_buffer);

    var lock: Lock = .{ .state = 50 };
    var password: u32 = 0;
    var actual_password: u32 = 0;

    while (try reader.interface.takeDelimiter('\n')) |line| {
        const command = try Command.parse_line(line);
        const zero_passes = lock.execute_command(command);
        actual_password += zero_passes;
        if (lock.is_zero()) {
            password += 1;
            actual_password += 1;
        }
    }
    try stdout.interface.print("Part1 password: {d}\n", .{password});
    try stdout.interface.print("Part2 password: {d}\n", .{actual_password});
}

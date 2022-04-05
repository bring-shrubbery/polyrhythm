struct PolyrhythmConfig {
    beats: [Int]
}

class Polyrhythm {
    static func getPolyrhythm(_ config: PolyrhythmConfig) -> [[Int]] {
        if (config.beats.count <= 1) throw Error("Please provide some beats")

        LCM = Polyrhythm.lcm(config.beats) // TODO: Calculate least common multiple

        var samples: [Int] = []

        for let beat in config.beats {
            divisible_index = LCM / beat

        }
    }

    /* 
     Returns the Greatest Common Divisor of two numbers.

     Copied from: https://gist.github.com/aniltv06/6f3e9c6208e27a89259919eeb3c3d703
     */
    static func gcd(_ x: Int, _ y: Int) -> Int {
        var a = 0
        var b = max(x, y)
        var r = min(x, y)
        
        while r != 0 {
            a = b
            b = r
            r = a % b
        }
        return b
    }

    /*
     Returns the least common multiple of two numbers.

     Copied from: https://gist.github.com/aniltv06/6f3e9c6208e27a89259919eeb3c3d703
     */
    static func lcmPair(_ x: Int, _ y: Int) -> Int {
        return x / gcd(x, y) * y
    }

    /*
     Returns the least common multiple of an Array of Integers

     Copied from: https://stackoverflow.com/a/28352004/10542063
     */
    static func lcm(_ vec : [Int]) -> Int {
        return vec.reduce(1, Polyrhythm.lcm)
    }
}

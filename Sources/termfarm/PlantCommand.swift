import ArgumentParser
import Foundation

struct Plant: ParsableCommand {
    static let configuration = CommandConfiguration(
        abstract: "Plant a seed"
    )

    @Argument(help: "Seed to plant")
    var seedID: String

    func run() throws {
        var farm = try loadFarm()

        guard cropRegistry[seedID] != nil else {
            throw ValidationError("Unknown seed: '\(seedID)'")
        }

        let seedCount = farm.inventory.seeds[seedID, default: 0]
        guard seedCount > 0 else {
            throw ValidationError("You don't have any \(seedID) seeds")
        }

        guard let index = farm.plots.firstIndex(where: { $0.plantedCrop == nil }) else {
            throw ValidationError("No available plots. Harvest some crops or purchase more plots from the Market")
        }

        farm.plots[index].plantedCrop  = seedID
        farm.plots[index].plantedAt    = Date()
        farm.inventory.seeds[seedID]! -= 1

        if farm.inventory.seeds[seedID] == 0 {
            farm.inventory.seeds.removeValue(forKey: seedID)        
        }

        farm.lastUpdated = Date()
        try saveFarm(farm)

        print("󰜐 Planted \(seedID)")
    }
}

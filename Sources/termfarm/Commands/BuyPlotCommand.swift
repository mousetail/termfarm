import ArgumentParser
import Foundation

struct BuyPlotCommand: ParsableCommand {
    static let configuration = CommandConfiguration(
        commandName: "buyplot",
        abstract: "Buy a new plot"
    )

    @Flag var showprice = false

    func run() throws {
        var farm = try loadFarm()

        let currentPlots = farm.plots.count
        let price        = nextPlotPrice(plotCount: currentPlots)

        if showprice {
            print("Plot \(currentPlots + 1) costs \(price) coins")
            return
        }

        guard farm.coins >= price else {
            throw ValidationError("Not enough coins. You have \(farm.coins), but need \(price) coins")
        }

        farm.coins -= price
        farm.plots.append(
            Plot(
                id: UUID(),
                plantedCrop: nil,
                plantedAt: nil
            )
        )

        farm.lastUpdated = Date()
        try saveFarm(farm)

        print("Bought new plot, total plots is now \(farm.plots.count)")
    }
}

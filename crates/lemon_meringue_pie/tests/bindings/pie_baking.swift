import Foundation
import LemonMeringuePie

public typealias PieError = Error

extension PieError: LocalizedError {
  public var errorDescription: String? {
    errorMessageFromError(error: self)
  }

  public var errorCode: UInt32 {
    errorCodeFromError(error: self)
  }
}

func test_baking() throws {
  let chef = try Chef(name: "Auguste Gusteau", money: Money(amount: 50))
  let farm = Farm(money: Money(amount: 200))
  let produce = farm.produce()
  assert(epochGetValue(epoch: produce.epoch) == 1337)
  let epoch = try newEpochFromStr(epoch: "1337")
  assert(produce.epoch == epoch)
  assert(produce.batch != farm.produce().batch)
  let kitchen = kitchenStockWith(produce: produce)
  let pie = bakeLemonMeringuePie(chef: chef, kitchen: kitchen)

  assert(chef.balance() == 50)
  assert(farm.balance() == 200)

  chefSellPieToFarm(chef: chef, pie: pie, farm: farm, price: 25)

  assert(chef.balance() == 75)
  assert(farm.balance() == 175)
}

func test_errors() {

  do {
    try newEpochFromStr(epoch: "not a number")
  } catch let error as PieError {
    assert(error.errorCode == 100)
    assert(error.errorDescription == "Invalid Epoch string: 'not a number'")
  } catch {
    fatalError("Unexpected error: \(error)")
  }

  do {
    try Chef(name: "Broke", money: Money(amount: 0))
  } catch let error as PieError {
    assert(error.errorCode == 101)
    assert(error.errorDescription == "Chef cannot be broke!!")
  } catch {
    fatalError("Unexpected error: \(error)")
  }

}

func test() throws {
  try test_baking()
  test_errors()
}

try! test()

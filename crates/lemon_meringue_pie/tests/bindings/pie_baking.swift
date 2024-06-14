import LemonMeringuePie

func test() {
  let chef = Chef(name: "Auguste Gusteau", money: Money(amount: 50))
  let farm = Farm(money: Money(amount: 200))
  let produce = farm.produce()
  assert(produce.batch != farm.produce().batch)
  let kitchen = kitchenStockWith(produce: produce)
  let pie = bakeLemonMeringuePie(chef: chef, kitchen: kitchen)

  assert(pie.batch != newPieBatchIdRandom())

  assert(chef.balance() == 50)
  assert(farm.balance() == 200)

  chefSellPieToFarm(chef: chef, pie: pie, farm: farm, price: 25)

  assert(chef.balance() == 75)
  assert(farm.balance() == 175)
}

test()

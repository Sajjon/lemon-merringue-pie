import com.sajjon.lemon.meringue.pie.*
import com.sajjon.chef.*
import com.sajjon.money.*
import com.sajjon.farm.*
import com.sajjon.pastry.*
import com.sajjon.meringue.*
import com.sajjon.lemon.filling.*
import com.sajjon.common.*
import com.sajjon.kitchen.*

fun test_bake_pie() {
    val chef = Chef(name = "Auguste Gusteau", money = Money(amount = 50u))
    val farm = Farm(money = Money(amount = 200u))
    val produce = farm.produce()
    val kitchen = kitchenStockWith(produce = produce)
    val pie = bakeLemonMeringuePie(chef = chef, kitchen = kitchen)
  
    assert(chef.balance() == 50.toULong())
    assert(farm.balance() == 200.toULong())
  
    chefSellPieToFarm(chef = chef, pie = pie, farm = farm, price = 25u)
  
    assert(chef.balance() == 75.toULong())
    assert(farm.balance() == 175.toULong())
}

fun test_bag_of_bytes() {
    val f0 = Fridge(
        egg = EggBox.TWELVE,
        butter = ButterBlock(weight = 1u),
        lemons = Lemons(count = 1u),
        firmwareHash = listOf(1.toUByte()),
        uuid = newApplianceIdDefault()
    )
    val f1 = Fridge(
        egg = EggBox.TWELVE,
        butter = ButterBlock(weight = 1u),
        lemons = Lemons(count = 1u),
        firmwareHash = listOf(1.toUByte()),
        uuid = newApplianceIdDefault()
    )
    assert(f0 == f1)

    var x = newBagOfBytesFromHexString(hex = "adbe")
    var y = newBagOfBytesFromHexString(hex = "adbe")
    x = newBagOfBytesPrependDe(bagOfBytes = x)
    x = newBagOfBytesAppendEf(bagOfBytes = x)
    y = newBagOfBytesAppendEf(bagOfBytes = y)
    y = newBagOfBytesPrependDe(bagOfBytes = y)
    assert(x == y)
    assert(bagOfBytesToHexString(bagOfBytes = x) == "deadbeef")
}

fun test() {
    test_bake_pie()
    test_bag_of_bytes()
}

test()
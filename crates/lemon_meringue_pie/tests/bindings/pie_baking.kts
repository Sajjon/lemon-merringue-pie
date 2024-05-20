import com.sajjon.lemon.meringue.pie.*
import com.sajjon.chef.*
import com.sajjon.money.*
import com.sajjon.farm.*
import com.sajjon.pastry.*
import com.sajjon.meringue.*
import com.sajjon.lemon.filling.*
import com.sajjon.kitchen.*

fun test() {
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

test()
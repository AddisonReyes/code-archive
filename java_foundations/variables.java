package java_foundations;

import java.util.ArrayList;

public class variables {
    public static void main(String[] args) {

        String name = "Addison";
        int age = 24;

        boolean male = true;

        name = name.concat(" Reyes");
        // System.out.println(name);
        // System.out.println(age);

        int health = 100;

        health += 20;
        health *= 20;
        health -= 20;
        health /= 20;

        // System.out.println(health);

        double accountBalance = 100.26;
        double cost = 50.0;
        boolean canAfford = accountBalance >= cost;

        // System.out.println(canAfford);

        int numLives = 3;
        boolean isAlive = numLives > 0 && health > 0;

        // System.out.println(isAlive);

        String[] inventory = { "knife", "Bread", "Helmet" };
        String knife = inventory[0];
        inventory[1] = "Fruit";

        String[] inventory2 = new String[3];

        ArrayList<String> inventory3 = new ArrayList<>();
        inventory3.add("Knife");
        inventory3.add("Bread");
        inventory3.add("Helmet");

        System.out.println(inventory3.get(0));

        inventory3.set(1, "Gloves");

    }
}

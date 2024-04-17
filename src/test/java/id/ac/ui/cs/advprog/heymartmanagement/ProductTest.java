package id.ac.ui.cs.advprog.heymartmanagement;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

public class ProductTest {
    private Product productA;

    @BeforeEach
    public void setUp() {
        productA = Product.builder()
                .name("Product A")
                .price(100000)
                .quantity(25)
                .build();
    }

    @Test
    void testProductBuilder() {
        Product product = new Product.builder()
                .name("Product A")
                .price(100000)
                .quantity(25)
                .build();

        assertEquals("Product A", product.getName());
        assertEquals(100000, product.getPrice());
        assertEquals(25, product.getQuantity);
    }

    @Test
    void testProductBuilderWithInvalidName() {
        assertThrows(IllegalArgumentException.class, () -> {
            Product.builder()
                    .name(null);
        });
        assertThrows(IllegalArgumentException.class, () -> {
            Product.builder()
                    .name("");
        });
        assertThrows(IllegalArgumentException.class, () -> {
            Product.builder()
                    .name(" ");
        });
        assertThrows(IllegalArgumentException.class, () -> {
            Product.builder()
                    .name(" Product A");
        });
        assertThrows(IllegalArgumentException.class, () -> {
            Product.builder()
                    .name("!@#$%^&*()");
        });
        assertThrows(IllegalArgumentException.class, () -> {
            Product.builder()
                    .name("!Product A");
        });
        assertThrows(IllegalArgumentException.class, () -> {
            Product.builder()
                    .name("The name of the product cannot exceed past the character limit " +
                            "of 100 that has been set by the creators of the app");
        });
    }

    @Test
    void testProductBuilderWithInvalidPrice() {
        assertThrows(IllegalArgumentException.class, () -> {
            Product.builder()
                    .price(null);
        });
        assertThrows(IllegalArgumentException.class, () -> {
            Product.builder()
                    .price(0);
        });
        assertThrows(IllegalArgumentException.class, () -> {
            Product.builder()
                    .price(999);
        });
        assertThrows(IllegalArgumentException.class, () -> {
            Product.builder()
                    .price(-1);
        });
        assertThrows(IllegalArgumentException.class, () -> {
            Product.builder()
                    .price("angka");
        });
    }

    void testProductBuilderWithInvalidQuantity() {
        assertThrows(IllegalArgumentException.class, () -> {
            Product.builder()
                    .quantity(null);
        });
        assertThrows(IllegalArgumentException.class, () -> {
            Product.builder()
                    .quantity(-1);
        });
        assertThrows(IllegalArgumentException.class, () -> {
            Product.builder()
                    .quantity(10001);
        });
        assertThrows(IllegalArgumentException.class, () -> {
            Product.builder()
                    .quantity("angka");
        });
    }
}

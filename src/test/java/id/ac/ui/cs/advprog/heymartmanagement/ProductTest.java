package id.ac.ui.cs.advprog.heymartmanagement;

import id.ac.ui.cs.advprog.heymartmanagement.model.Product;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

public class ProductTest {
    private Product productA;

    @BeforeEach
    public void setUp() {
        productA = Product.builder()
                .productName("Product A")
                .productPrice(100000)
                .productQuantity(25)
                .build();
    }

    @Test
    void testProductBuilder() {
        Product product = Product.builder()
                .productName("Product A")
                .productPrice(100000)
                .productQuantity(25)
                .build();

        assertEquals("Product A", product.getProductName());
        assertEquals(100000, product.getProductPrice());
        assertEquals(25, product.getProductQuantity());
    }

    @Test
    void testProductBuilderWithInvalidName() {
        assertThrows(IllegalArgumentException.class, () -> {
            Product.builder()
                    .productName(null);
        });
        assertThrows(IllegalArgumentException.class, () -> {
            Product.builder()
                    .productName("");
        });
        assertThrows(IllegalArgumentException.class, () -> {
            Product.builder()
                    .productName(" ");
        });
        assertThrows(IllegalArgumentException.class, () -> {
            Product.builder()
                    .productName(" Product A");
        });
        assertThrows(IllegalArgumentException.class, () -> {
            Product.builder()
                    .productName("!@#$%^&*()");
        });
        assertThrows(IllegalArgumentException.class, () -> {
            Product.builder()
                    .productName("!Product A");
        });
        assertThrows(IllegalArgumentException.class, () -> {
            Product.builder()
                    .productName("The name of the product cannot exceed past the character limit " +
                            "of 100 that has been set by the creators of the app");
        });
    }

    @Test
    void testProductBuilderWithInvalidPrice() {
        assertThrows(IllegalArgumentException.class, () -> {
            Product.builder()
                    .productPrice(0);
        });
        assertThrows(IllegalArgumentException.class, () -> {
            Product.builder()
                    .productPrice(999);
        });
        assertThrows(IllegalArgumentException.class, () -> {
            Product.builder()
                    .productPrice(-1);
        });
    }

    void testProductBuilderWithInvalidQuantity() {
        assertThrows(IllegalArgumentException.class, () -> {
            Product.builder()
                    .productQuantity(-1);
        });
        assertThrows(IllegalArgumentException.class, () -> {
            Product.builder()
                    .productQuantity(10001);
        });
    }
}

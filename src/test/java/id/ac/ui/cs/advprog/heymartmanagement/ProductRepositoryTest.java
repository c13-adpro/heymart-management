package id.ac.ui.cs.advprog.heymartmanagement;

import static org.junit.jupiter.api.Assertions.*;
import static org.mockito.Mockito.*;

import id.ac.ui.cs.advprog.heymartmanagement.model.Product;
import id.ac.ui.cs.advprog.heymartmanagement.repository.ProductRepository;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;
import org.mockito.InjectMocks;
import org.mockito.Mock;
import org.mockito.junit.jupiter.MockitoExtension;

import java.util.UUID;


@ExtendWith(MockitoExtension.class)
public class ProductRepositoryTest {

    private ProductRepository productRepository;

    @BeforeEach
    void setUp() {
        productRepository = new ProductRepository();
    }

    @Test
    void testCreateAndFind() {
        // Arrange
        Product product = new Product(UUID.randomUUID().toString(), "Sampo Cap Bambang", 100L, 100);

        // Act
        productRepository.create(product);
        Product foundProduct = productRepository.findByProductId(product.getProductId());

        // Assert
        assertNotNull(foundProduct);
        assertEquals(product, foundProduct);
    }

    @Test
    void testFindByProductId_NotFound() {
        // Arrange
        String nonExistentProductId = UUID.randomUUID().toString();

        // Act
        Product foundProduct = productRepository.findByProductId(nonExistentProductId);

        // Assert
        assertNull(foundProduct);
    }
}
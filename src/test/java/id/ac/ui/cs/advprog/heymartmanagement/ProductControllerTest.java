package id.ac.ui.cs.advprog.heymartmanagement;

import id.ac.ui.cs.advprog.heymartmanagement.controller.ProductController;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.InjectMocks;
import org.mockito.Mock;
import org.mockito.MockitoAnnotations;
import org.springframework.ui.Model;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.mockito.Mockito.*;


class ProductControllerTest {
    @Mock
    private Model model;

    @InjectMocks
    private ProductController productController;

    @BeforeEach
    void setUp() {
        MockitoAnnotations.openMocks(this);
    }

    @Test
    void testCreateProductPage() {
        String expectedViewName = "createProduct";
        String actualViewName = productController.createProductPage(model);
        assertEquals(expectedViewName, actualViewName);
    }

}
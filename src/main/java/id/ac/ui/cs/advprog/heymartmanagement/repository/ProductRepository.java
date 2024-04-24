package id.ac.ui.cs.advprog.heymartmanagement.repository;
import java.util.HashMap;
import java.util.Map;
import id.ac.ui.cs.advprog.heymartmanagement.model.Product;

public class ProductRepository {
    private final Map<String, Product> products;

    public ProductRepository() {
        this.products = new HashMap<>();
    }

    public void create(Product product) {
        products.put(product.getProductId(), product);
    }

    public Product findByProductId(String productId) {
        return products.get(productId);
    }
}
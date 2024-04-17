package id.ac.ui.cs.advprog.heymartmanagement.repository;

import id.ac.ui.cs.advprog.heymartmanagement.model.Product;

import java.util.ArrayList;
import java.util.Iterator;
import java.util.List;
import java.util.UUID;

public class ProductRepository {
    private List<Product> productData = new ArrayList<>();


    public Product createProduct(Product product) {
        if (product.getProductId() == null) {
            UUID uuid = UUID.randomUUID();
            product.setProductId(uuid.toString());
        }

        productData.add(product);
        return product;
    }

    public Iterator<Product> findAll() {
        return productData.iterator();
    }

    public Product findById(String productId) {
        return productData.stream()
                .filter(product -> product.getProductId().equals(productId))
                .findFirst()
                .orElseThrow(() ->
                        new IllegalArgumentException("Invalid product Id:" + productId)
                );
    }
}

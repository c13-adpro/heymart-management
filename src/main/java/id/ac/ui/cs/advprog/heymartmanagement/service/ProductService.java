package id.ac.ui.cs.advprog.heymartmanagement.service;

import id.ac.ui.cs.advprog.heymartmanagement.model.Product;

import java.util.List;

public interface ProductService {
    Product createProduct(Product product);
    List<Product> getAllProducts();

}
